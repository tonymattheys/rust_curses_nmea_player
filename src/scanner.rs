use crate::udp_broadcaster::where_am_i_now;
use chrono::NaiveDate;
use geoutils::Location;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn scan_and_report(file_h: File) -> bool {
    let mut where_have_i_been: Vec<String> = [].to_vec();
    let mut lat_d: f64 = 0.0;
    let mut lon_d: f64 = 0.0;
    let mut accum_distance: f64 = 0.0;
    let mut file_start_time = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let mut dt = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    // Read the file line by line and send each line over UDP
    let reader = io::BufReader::new(file_h);
    for line in reader.lines() {
        let line = line.unwrap_or(" ".to_string());
        let fields: Vec<&str> = line.split(',').collect();
        if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("ZDA") {
            let y: i32 = FromStr::from_str(fields[4]).unwrap_or(1970);
            let m: u32 = FromStr::from_str(fields[3]).unwrap_or(1);
            let d: u32 = FromStr::from_str(fields[2]).unwrap_or(1);
            let mut hr: u32 = FromStr::from_str(&fields[1][0..2]).unwrap_or(0);
            let mut mn: u32 = FromStr::from_str(&fields[1][2..4]).unwrap_or(0);
            let mut se: u32 = FromStr::from_str(&fields[1][4..6]).unwrap_or(0);
            // Some GPS units will give you "60" for minutes or seconds but chrono hates that
            // Quick and dirty fix for that...
            if se >= 60 {
                se = 0;
                mn += 1
            }
            if mn >= 60 {
                mn = 0;
                hr += 1
            }
            // The most recent date that we read from the file is always in 'dt'
            dt = NaiveDate::from_ymd_opt(y, m, d)
                .unwrap()
                .and_hms_opt(hr, mn, se)
                .unwrap();
            // If we have not yet initialized the start times, then do it now.
            if file_start_time
                == NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
            {
                file_start_time = dt;
            }
        }
        // $GPGGA,020659.21,4937.8509,N,12401.4384,W,2,9,0.83,,M,,M*44
        if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("GGA") {
            // Make our first distance point the last location stored in lat_d, lon_d
            let mut first_time: bool = false;
            if lat_d == 0.0 && lon_d == 0.0 {
                first_time = true;
            };
            let l1 = Location::new(lat_d, lon_d);
            // Get latitude from GPS statement
            let x: f64 = FromStr::from_str(&fields[2]).unwrap_or(0.0);
            let lat_deg: f64 = (x / 100.0).floor();
            let lat_min: f64 = (x / 100.0).fract() * 100.0;
            let n_s: &str = fields[3];
            lat_d = lat_deg + (lat_min / 60.0);
            if n_s.contains("S") {
                lat_d = lat_d * -1.0
            };
            // Get longitude from GPS statements
            let x: f64 = FromStr::from_str(&fields[4]).unwrap_or(0.0);
            let lon_deg: f64 = (x / 100.0).floor();
            let lon_min: f64 = (x / 100.0).fract() * 100.0;
            let e_w: &str = fields[5];
            lon_d = lon_deg + (lon_min / 60.0);
            if e_w.contains("W") {
                lon_d = lon_d * -1.0
            };
            // By here we will have some meaningful lat/lon values except
            // the first time through when lat/long are zero
            if !first_time {
                let l2 = Location::new(lat_d, lon_d);
                let d = accum_distance + l1.haversine_distance_to(&l2).meters();
                accum_distance = d;
            }
            if !where_have_i_been.contains(&&where_am_i_now::from_http(lat_d, lon_d)) {
                where_have_i_been.push(where_am_i_now::from_http(lat_d, lon_d));
                println!(
                    "At {} UTC, ({:.4}, {:.4}) is near '{}'",
                    fields[1],
                    lat_d,
                    lon_d,
                    where_am_i_now::from_http(lat_d, lon_d)
                );
                println!(
                    "Distance travelled so far is {:.1} nautical miles, or {:.1} km",
                    accum_distance / 1000.0 * 0.5399568,
                    accum_distance / 1000.0
                );
            }
        }
    }
    println!(
        "Start time in file is {} UTC",
        file_start_time.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    println!(
        "Last time read from file is {} UTC",
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    println!(
        "Accumulated distance travelled in this file is {:.1} nautical miles, or {:.1} km",
        accum_distance/ 1000.0 * 0.5399568,
        accum_distance / 1000.0
    );

    true
}
