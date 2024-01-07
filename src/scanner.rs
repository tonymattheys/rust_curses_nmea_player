use chrono::NaiveDate;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn scan_and_report(f: File) -> bool {
    let mut gpzda: u64 = 0;
    let mut file_start_time = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let mut dt = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    // Read the file line by line and send each line over UDP
    let reader = io::BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap_or(" ".to_string());
        let fields: Vec<&str> = line.split(',').collect();
        if fields[0].starts_with("$") && fields[0].len() >= 6 && fields[0][3..6].eq("ZDA") {
            gpzda += 1;
            let y: i32 = FromStr::from_str(fields[4]).unwrap_or(1970);
            let m: u32 = FromStr::from_str(fields[3]).unwrap_or(1);
            let d: u32 = FromStr::from_str(fields[2]).unwrap_or(1);
            let mut hr: u32 = FromStr::from_str(&fields[1][0..2]).unwrap_or(0);
            let mut mn: u32 = FromStr::from_str(&fields[1][2..4]).unwrap_or(0);
            let mut se: u32 = FromStr::from_str(&fields[1][4..6]).unwrap_or(0);
			// Some GPS units will give you "60" for minutes or seconds but chrono hates that
			// Quick and dirty fix for that...
            if se >=60 { se=0; mn += 1 }
            if mn >=60 { mn=0; hr += 1 }
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
    }
    println!("Number of $GPZDA sentences {}", gpzda);
    println!(
        "Start time in file is {}",
        file_start_time.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    println!(
        "Last time read from file is {}",
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    true
}
