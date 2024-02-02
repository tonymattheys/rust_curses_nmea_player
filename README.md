# Curses-based NMEA file player written in Rust

Rust program which will play the contents of a NMEA text file over the network. The program will try to 
use the times contained in the NMEA sentences to keep itself more or less in synch in real time with the
NMEA times. This relies on your file having $GPZDA sentences present so if your GPS doesn't generate them
then time synchronization will not work exactly. However, the program also introduces delays to account for
sending NMEA sentences over a "real" NMEA bus at 38,400 baud so the times might end up being quite close
anyway. Your mileage may vary.

# How to run
- cd wherever_you_downloaded_the_program
- cargo run [--release]

# How to build an executable
- cd wherever_you_downloaded_the_program
- cargo build --release
will build an executable called "nmea_player" under ./target/release

# Description (also available with --help option)
This program will read a file specified by the user and perform various operations
using the contents of the file as input. The most common way to use this program is
to read in a NMEA0183 file and re-send the NMEA sentences out onto the network using
UDP broadcast on port 10110. This will appear to be a Comar system to Navionics and
other navigation systems that listen for UDP broadcasts on the network.

The program can also scan the given file and produce a report showing summary information
about the NMEA sentences contained therein. For example, it will report on time stamps
found in sentences like $GPZDA, which will, in turn allow the user to ask the program to
start broadcasting over the network starting at a certain time in the file. This is very 
useful when analyzing sailboat races, for example, where there could be a lot of unwanted 
NMEA traffic before and after the race itself.

# Command line options
Usage: nmea_player [OPTIONS] --file <NMEA_FILE>

Options:
-  -s, --scan
-  -t, --time <hh:mm:ss[.ss]>	[default: 00:00:00]
-  -u, --udp <UDP_PORT>			[default: 10110]
-  -i, --if <en0, eth0 ... etc>	[default: eth0]
-  -f, --file <NMEA_FILE>
-  -h, --help 			Print help (see a summary with '-h')
-  -V, --version			Print version
