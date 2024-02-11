use chrono::{Utc, NaiveDate};
use pnet::datalink::NetworkInterface;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::thread::sleep;

pub(crate) mod where_am_i_now;
mod screen;

pub fn send_lines(file: File, interface: NetworkInterface, udp_port: u16, _start_time: String,) -> io::Result<()> {
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
    // Read the file line by line and send each line over UDP
    let reader = io::BufReader::new(file.try_clone()?);
	// Define some variables that can store various dates/times that we need to keep 
	// packet sending in synch (more or less) with real time
	let mut file_start_time = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
	let mut locl_start_time = Utc::now().naive_utc();
	let mut sleep_time = locl_start_time - locl_start_time ;
    let mut dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
	// Variables that store strings that will be displayed on the screen
	let mut lat_d: f64 = 0.0;
	let mut lat_s: String = "".to_string();
	let mut lon_d: f64 = 0.0;
	let mut lon_s: String = "".to_string();
	let mut cog: String = "".to_string();
	let mut sog: String = "".to_string();
	let mut dpt: String = "".to_string();
	let mut wnd: String = "".to_string();
    let mut whr = "".to_string();
	// Iterate through the lines of the file and process each line as we see it.
	// For certain types of sentences we parse the line and extract some information
	// that we need from its fields.
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
			// We put locl_start_time as the default for the unwrap() to help prevent panics
            dt = NaiveDate::from_ymd_opt(y, m, d).unwrap_or(locl_start_time.date()).and_hms_opt(hr, mn, se).unwrap_or(locl_start_time);
			// If we have not yet initialized the start times, then do it now.
            if file_start_time == NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap() {
            	file_start_time = dt;
            	locl_start_time = Utc::now().naive_utc();
            }
	        // Resynch the elapsed time clocks by sleeping before reading the next line
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
			lat_d = lat_deg + (lat_min / 60.0);
			if n_s.contains("S") {lat_d = lat_d * -1.0 }
			lat_s = format!("{:3}° {:2.4} {} ({:.4})", lat_deg, lat_min, n_s, lat_d);
			// Get longitude from GPS statements
			let x: f64 = FromStr::from_str(&fields[4]).unwrap_or(0.0) ;
			let lon_deg: f64 = (x / 100.0).floor();
			let lon_min: f64 = (x / 100.0).fract() * 100.0 ;
			let e_w: &str  = fields[5];
			lon_d = lon_deg + (lon_min / 60.0);
			if e_w.contains("W") {lon_d = lon_d * -1.0 }
			lon_s = format!("{:3}° {:2.4} {} ({:.4})", lon_deg, lon_min, e_w, lon_d);
		}
		// $IIVTG,359.5,T,,M,0.1,N,0.1,K,D*15
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("VTG") {
			let c: f64 = FromStr::from_str(&fields[1]).unwrap_or(0.0) ;
			cog = format!("{:3.0} °T", c);
			let s: f64 = FromStr::from_str(&fields[5]).unwrap_or(0.0) ;
			sog = format!("{:2.1} kts", s);
		}
		// $WIVWR,31.7,L,0.5,N,0.3,M,0.9,K*73
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("VWR") {
			let a: f64 = FromStr::from_str(&fields[1]).unwrap_or(0.0) ;
			let d = fields[2] ;
			let v: f64 = FromStr::from_str(&fields[3]).unwrap_or(0.0) ;
			wnd = format!("{:3.0} degrees {} at {:2.1} knots", a, d, v);
		}
		// $SDDPT,10.38,0,*6F
		if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("DPT") {
			let d: f64 = FromStr::from_str(&fields[1]).unwrap_or(0.0) ;
			let o: f64 = FromStr::from_str(&fields[2]).unwrap_or(0.0) ;
			dpt = format!("{:3.1} m", d + o);
		}
        // Inject a short delay to account for sending the line at 4800 baud
        // 4800 baud is 600 bytes/sec so delay (in msec) is line.len()/600*1000
        // If sleep_time is negative it means that we are slower in real time 
        // than the GPS time in the file and we don't sleep at all. This allows 
        // the program time to "catch up" to the GPS time stamps in the file.
        let mut dly: f64 = line.len() as f64 / 600.0 * 1000.0;
       	if sleep_time.num_milliseconds() <= 0 {
	    	dly = 0.0;
	    }
	    if ((Utc::now().naive_utc() - locl_start_time).num_seconds() % 30) <= 1 {
	    	whr = where_am_i_now::wicked_fast(lat_d, lon_d);
	    }
        let msg = format!("Delay added to account for baud rate = {:4} ms", dly.floor() as u64);
       	sleep(std::time::Duration::from_millis(dly.floor() as u64));
		// Now repaint the screen and send the line on the socket.
		screen::paint(&window, file_start_time, locl_start_time, dt, sleep_time, &lat_s, &lon_s, &cog, &sog, &dpt, &wnd, &whr, &msg);
        socket.send_to(format!("{}\r\n", line).as_bytes(), &destination)?;
    }
    screen::window_cleanup(&window);
    println!("File lines echoed on interface '{}' UDP port {}.", interface.name, udp_port);
    Ok(())
}
