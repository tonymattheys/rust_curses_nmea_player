use clap::Parser;
use pnet::datalink::{self};
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use std::process::exit;

mod udp_broadcaster;
mod scanner;


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
    #[arg(short, long, default_value_t = false, value_name = "true | false (default is false)")]
    scan: bool,

    #[arg(short, long, default_value = "00:00:00", value_name = "hh:mm:ss[.ss]")]
    time: String,

    #[arg(short, long="udp", default_value_t = 10110, value_name = "UDP_PORT")]
    udp_port: u16,

    #[arg(short, long="if", default_value = "eth0", value_name = "en0, eth0 ... etc")]
    if_name: String,

    #[arg(short, long="file", value_name = "NMEA_FILE")]
    file_name: PathBuf,
}

fn main() -> io::Result<()> {
    // Parse command-line arguments to get the network interface name and file name
    let cli = Cli::parse();

    // Open the file
    let file = File::open(cli.file_name)?;
	// If he asked for the file to be scanned, then just pass the file handle to the scanner
    if cli.scan {
    	if scanner::scan_and_report(file) {
    		exit(0)
    	} else {
    		exit(2)
    	}
    }

    let start_time = cli.time;
    let if_name = cli.if_name;

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
    udp_broadcaster::send_lines(file, interface, cli.udp_port, start_time)?;
    Ok(())
}
