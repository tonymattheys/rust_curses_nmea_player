use chrono::{Utc, NaiveDateTime, Duration};
use pancurses::{noecho, initscr, endwin, Input::Character, Window, A_REVERSE};
use std::process::exit;

pub fn new() -> Window {
    // Initialize curses
    initscr()
}

pub fn window_cleanup(win: &Window) -> bool {
    win.refresh();
    win.clear();
    endwin();
    true
}

pub fn paint(window: &Window, fst: NaiveDateTime, lst: NaiveDateTime, dt: NaiveDateTime, sleep: Duration, lat_s: &str, lon_s: &str, cog: &str, sog: &str, dpt: &str, wnd: &str, loc: &str, msg: &str,) -> bool {
    // Start Date and Time for file and local clock
    window.mv(0, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("File Start :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(fst.format("%Y-%m-%d %H:%M:%S").to_string());
    window.addstr(" UTC");
    window.mv(0, 40);
    window.attron(A_REVERSE);
    window.addstr("Local Start :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(lst.format("%Y-%m-%d %H:%M:%S").to_string());
    window.addstr(" UTC");
    // Date and Time
    window.mv(1, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("File Time  :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(dt.to_string());
    window.addstr(" UTC");
    window.mv(1, 40);
    window.attron(A_REVERSE);
    window.addstr("Local Time  :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(format!("{}", &Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S")));
    window.addstr(" UTC");
    window.mv(2, 0);
    window.clrtoeol();
    window.addstr("Difference between real elapsed time and file elapsed time = ");
    window.addstr(sleep.num_milliseconds().to_string());
    window.addstr(" ms");
    // Latitude
    window.mv(4, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Latitude");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(lat_s.to_string());
    // Longitude
    window.mv(4, 40);
    window.attron(A_REVERSE);
    window.addstr("Longitude");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(lon_s.to_string());
    // COG and SOG
    window.mv(6, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("COG:");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(cog.to_string());
    window.mv(6, 40);
    window.attron(A_REVERSE);
    window.addstr("SOG:");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(sog.to_string());
    // Depth
    window.mv(8, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Depth :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(dpt.to_string());
    // Wind
    window.mv(10, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Wind :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(wnd.to_string());
    // Location and Random message
    window.mv(12, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Location :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(loc.to_string());
    window.mv(13, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Message  :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(msg.to_string());
    
    // Cursor back to home position
    window.mv(0, 0);
    window.nodelay(true);
    noecho(); // set terminal echo mode off

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
    true
}
