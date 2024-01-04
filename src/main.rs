use chrono::{Utc, NaiveDate, NaiveDateTime};
use clap::Parser;
use pnet::datalink::{self, NetworkInterface};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;

// This declaration will look for a file named `screen.rs` and will
// insert its contents inside a module named `screen` under this scope
mod screen;


#[derive(Parser)]
#[command(author, version)] // Read from `Cargo.toml`
#[command(
    about = "A Rust program to read a text file containing NMEA sentences and resend them across the network."
)]
#[command(
    long_about = "This program will read a file specified by the user and perform various operations
using the contents of the file as input. The most common way to use this program is
to read in a NMEA0183 file and resend the NMEA sentences out onto the network using
UDP broadcast on port 10110. This will appear to be a Comar system to Navionics and
other navigation systems that listend for UDP broadcasts on the network.
\n
The program can also scan the given file and produce a report showing summary information
about the NMEA sentences contained therein. For example, it will report on time stamps
found in sentences like $GPZDA, which will, in turn allow the user to ask the program to
start broadcasting over the network starting at a certain time in the file. This is very 
useful when analyzing sailboat races, for example, where there could be a lot of unwanted 
NMEA traffic before and after the race itself."
)]
struct Cli {
    #[arg(short, long, value_name = "hh:mm:ss[.ss]")]
    start_time: Option<String>,

    #[arg(short, long, default_value_t = 10110, value_name = "UDP_PORT")]
    udp_port: u16,

    #[arg(short, long, value_name = "en0, eth0 ... etc")]
    if_name: String,

    #[arg(short, long, value_name = "NMEA_FILE")]
    file_name: PathBuf,
}

fn main() -> io::Result<()> {
    // Parse command-line arguments to get the network interface name and file name
    let cli = Cli::parse();
    let mut _start_time: String = cli.start_time.unwrap_or("99:99:99".to_string());
    let if_name = cli.if_name;

    // Open the file
    let file = File::open(cli.file_name)?;

    // Get the network interface with the name that was specified as the first parameter
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == if_name)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Interface '".to_owned() + &if_name + "' not found",
            )
        })?;
    // Read the file line by line and send each line over UDP to the specified interface
    send_lines(file, interface, cli.udp_port, _start_time)?;
    Ok(())
}

fn send_lines(file: File, interface: NetworkInterface, udp_port: u16, _start_time: String,) -> io::Result<()> {
	// Grab the broadcast address of the first IP address assigned to the specified interface
	let ip_addr = interface.ips[0].broadcast();
    let destination = SocketAddr::new(ip_addr, udp_port);
    // Open a UDP socket for the interface
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    // allow broadcasting on this socket...
    socket.set_broadcast(true)?;

    // Initialize curses
    let window: pancurses::Window = screen::new();
    window.clear();

    // Read the file line by line and send each line over UDP with a one-second delay
    let reader = io::BufReader::new(file.try_clone()?);
	
	let mut file_start_time = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
	let mut locl_start_time = Utc::now().naive_utc();
	let mut sleep_time = locl_start_time - locl_start_time ;
	
    let mut dt: NaiveDateTime = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
	let mut lat: String = "".to_string();
	let mut lon: String = "".to_string();
	let mut cog: String = "".to_string();
	let mut sog: String = "".to_string();
	let mut dpt: String = "".to_string();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        // $GPZDA,234626.99,22,02,2021,08,00*6A
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("ZDA") {
			let y: i32  = FromStr::from_str(fields[4]).unwrap_or(1970);
			let m: u32  = FromStr::from_str(fields[3]).unwrap_or(1);
			let d: u32  = FromStr::from_str(fields[2]).unwrap_or(1);
			let hr: u32 = FromStr::from_str(&fields[1][0..2]).unwrap_or(0);
			let mn: u32 = FromStr::from_str(&fields[1][2..4]).unwrap_or(0);
			let se: u32 = FromStr::from_str(&fields[1][4..6]).unwrap_or(0);
            dt = NaiveDate::from_ymd_opt(y, m, d).unwrap().and_hms_opt(hr, mn, se).unwrap();
			// If we have not yet initialized the start times, then do it now.
            if file_start_time == NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap() {
            	file_start_time = dt;
            	locl_start_time = Utc::now().naive_utc();
            }
	        // Resynch the clocks by sleeping before reading the next line
    	    sleep_time = (dt - file_start_time) - (Utc::now().naive_utc() - locl_start_time);
        	if sleep_time.num_milliseconds() > 0 {
	        	sleep(std::time::Duration::from_millis(sleep_time.num_milliseconds() as u64));
        	}
		}
		// $GPGGA,020659.21,4937.8509,N,12401.4384,W,2,9,0.83,,M,,M*44
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("GGA") {
			// Get latitude from GPS statement
			let x: f64 = FromStr::from_str(&fields[2]).unwrap_or(0.0) ;
			let lat_deg: f64 = (x / 100.0).floor();
			let lat_min: f64 = (x / 100.0).fract() * 100.0 ;
			let n_s: &str  = fields[3];
			lat = format!("{:.0}°{:.4} {}", lat_deg, lat_min, n_s);
			// Get longitude from GPS statements
			let x: f64 = FromStr::from_str(&fields[4]).unwrap_or(0.0) ;
			let lon_deg: f64 = (x / 100.0).floor();
			let lon_min: f64 = (x / 100.0).fract() * 100.0 ;
			let e_w: &str  = fields[5];
			lon = format!("{:.0}°{:.4} {}", lon_deg, lon_min, e_w);
		}
		// $IIVTG,359.5,T,,M,0.1,N,0.1,K,D*15
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("VTG") {
			let c: f64 = FromStr::from_str(&fields[1]).unwrap_or(0.0) ;
			cog = format!("{:.1} °T", c);
			let s: f64 = FromStr::from_str(&fields[5]).unwrap_or(0.0) ;
			sog = format!("{:.1} kts", s);
		}
		// $SDDPT,10.38,0,*6F
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("DPT") {
			let d: f64 = FromStr::from_str(&fields[1]).unwrap_or(0.0) ;
			let o: f64 = FromStr::from_str(&fields[2]).unwrap_or(0.0) ;
			dpt = format!("{:.1} m", d + o);
		}
		screen::paint(&window, dt, sleep_time, &lat, &lon, &cog, &sog, &dpt);
        socket.send_to(line.as_bytes(), &destination)?;
    }
    screen::window_cleanup(&window);
    println!(
        "File lines echoed on interface '{}' UDP port {} with one-second delay.",
        interface.name, udp_port
    );
    Ok(())
}
