use chrono::prelude::*;
use gtk::prelude::*;
use serde::Deserialize;
use socket2::{Domain, Socket, Type};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
struct GpsData {
    #[serde(rename = "GGA")]
    gga: Option<GgaData>,
    #[serde(rename = "RMC")]
    rmc: Option<RmcData>,
}

#[derive(Debug, Deserialize)]
struct GgaData {
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "Latitude")]
    latitude: String,
    #[serde(rename = "Longitude")]
    longitude: String,
}

#[derive(Debug, Deserialize)]
struct RmcData {
    #[serde(rename = "Time")]
    time: String,
}

fn main() {
    // Set up GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Set up the main window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("GPS Tracker");
    window.set_default_size(800, 600);

    // Set up the web view for displaying the map
    let web_view = gtk::WebView::new();
    window.add(&web_view);

    // Create a shared state for GPS data
    let gps_data = Arc::new(Mutex::new(None));

    // Set up UDP socket
    let socket = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 10110);
    socket.bind(&addr.into()).unwrap();

    // Create a file chooser dialog
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Select NMEA File"),
        Some(&window),
        gtk::FileChooserAction::Open,
    );
    file_chooser.add_button("Open", gtk::ResponseType::Ok.into());
    file_chooser.add_button("Cancel", gtk::ResponseType::Cancel.into());

    // Handle file chooser dialog response
    let gps_data_clone = Arc::clone(&gps_data);
    let web_view_clone = web_view.clone();
    file_chooser.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Ok.into() {
            if let Some(file) = dialog.get_file() {
                if let Some(file_path) = file.get_path() {
                    start_gps_tracking(file_path, gps_data_clone.clone(), web_view_clone.clone());
                }
            }
        }
        dialog.close();
    });

    // Show the file chooser dialog when the window is first displayed
    window.connect_show(move |_| {
        file_chooser.run();
    });

    // Set up the GTK main loop
    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all components and start the GTK main loop
    window.show_all();
    gtk::main();
}

fn start_gps_tracking(file_path: std::path::PathBuf, gps_data: Arc<Mutex<Option<GpsData>>>, web_view: gtk::WebView) {
    // Your existing GPS tracking logic goes here, but use `file_path` to read from the selected file.
    // This function can be modified to suit your needs.
    // For simplicity, I'm omitting the actual file reading and processing logic.
    // You can replace the following line with your logic for reading NMEA sentences from the file.

    // Dummy function to read from the file (replace with actual logic)
    read_from_file(file_path);

    // Update the GUI with GPS data every second
    gtk::timeout_add_seconds_local(1, move || {
        let gps_data = gps_data.lock().unwrap().clone();
        if let Some(gps_data) = gps_data {
            update_gui(&web_view, &gps_data);
        }
        Continue(true)
    });
}

fn read_from_file(file_path: std::path::PathBuf) {
    // Replace this with your actual logic to read NMEA sentences from the file.
    // For simplicity, I'm using a dummy function that prints the file path.
    println!("Reading from file: {:?}", file_path);
}
