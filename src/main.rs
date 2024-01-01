use clap::Parser;
use pancurses::{echo, endwin, initscr, Input::Character, A_REVERSE};
use pnet::datalink::{self, NetworkInterface};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

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
    #[arg(short, long, value_name = "hh:mm:ss.ss")]
    start_time: Option<String>,

    #[arg(short, long, default_value_t = 10110, value_name = "UDP_PORT")]
    udp_port: u16,

    #[arg(short, long, value_name = "en0, eth0 ... etc")]
    if_name: String,

    #[arg(short, long, value_name = "NMEA_FILE")]
    file_name: PathBuf,
}

fn window_cleanup(win: pancurses::Window) -> bool {
    win.refresh();
    win.clear();
    endwin();
    true
}

fn main() -> io::Result<()> {
    // Parse command-line arguments to get the network interface name and file name
    let cli = Cli::parse();

    let mut _start_time: String = "99:99:99".to_string();
    if let Some(st) = cli.start_time {
        _start_time = st;
    };

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
    if let Some(ips) = interface.ips.into_iter().next() {
        let destination = SocketAddr::new(ips.broadcast(), udp_port);
        // Open a UDP socket for the interface
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        // Initialize curses
        let window = initscr();
        window.clear();

        // Read the file line by line and send each line over UDP with a one-second delay
        let reader = io::BufReader::new(file.try_clone()?);
        for line in reader.lines() {
            let line = line?;
            socket.send_to(line.as_bytes(), &destination)?;
            if line.contains("GPZDA") {
                let mut lcl_time: i32 = 0;
                let fields: Vec<&str> = line.split(',').collect();
                match fields[1][0..2].parse::<i32>() {
                    Ok(parsed_num) => {
                        lcl_time = parsed_num - 8;
                        if lcl_time < 0 {
                            lcl_time = 24 - lcl_time;
                        }
                    }
                    Err(_) => {
                        println!("Failed to parse the string as an integer");
                    }
                }
                window.mv(2, 2);
                window.clrtoeol();
                window.attron(A_REVERSE);
                window.addstr("Time");
                window.attroff(A_REVERSE);
                window.mv(2, 7);
                window.addstr(" (");
                window.addstr(lcl_time.to_string());
                window.addstr(") ");
                window.addstr(fields[1][0..2].to_string());
                window.addstr(":");
                window.addstr(fields[1][2..4].to_string());
                window.addstr(":");
                window.addstr(fields[1][4..].to_string());
                window.mv(0, 0);
                window.nodelay(true);
                echo(); // set terminal echo mode on

                let char = window.getch();
                match char {
                    Some(x) => {
                        if x == Character('q') {
                            window_cleanup(window);
                            exit(0);
                        }
                    }
                    None => {}
                }
                window.refresh();
            }
            // Wait before reading the next line
            sleep(Duration::from_millis(5));
        }
	    window_cleanup(window);
    }
    println!("File lines echoed on interface '{}' UDP port {} with one-second delay.", interface.name, udp_port);
    Ok(())
}
