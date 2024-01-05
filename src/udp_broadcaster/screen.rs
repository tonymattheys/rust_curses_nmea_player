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

pub fn paint(window: &Window, fst: NaiveDateTime, lst: NaiveDateTime, dt: NaiveDateTime, sleep: Duration, lat: &str, lon: &str, cog: &str, sog: &str, dpt: &str, wnd: &str, msg: &str,) -> bool {
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
    window.addstr(" UTC  (Offset = ");
    window.addstr(sleep.num_milliseconds().to_string());
    window.addstr(" ms )");
    // Latitude
    window.mv(3, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Latitude");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(lat.to_string());
    // Longitude
    window.mv(3, 40);
    window.attron(A_REVERSE);
    window.addstr("Longitude");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(lon.to_string());
    // COG and SOG
    window.mv(5, 0);
    window.attron(A_REVERSE);
    window.addstr("COG:");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(cog.to_string());
    window.mv(5, 40);
    window.attron(A_REVERSE);
    window.addstr("SOG:");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(sog.to_string());
    // Depth
    window.mv(7, 0);
    window.attron(A_REVERSE);
    window.addstr("Depth :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(dpt.to_string());
    // Wind
    window.mv(9, 0);
    window.attron(A_REVERSE);
    window.addstr("Wind :");
    window.attroff(A_REVERSE);
    window.addstr(" ");
    window.addstr(wnd.to_string());
    // Random message
    window.mv(11, 0);
    window.attron(A_REVERSE);
    window.addstr("Message :");
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
