use chrono::NaiveDateTime;
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

pub fn paint(window: &Window, dt: NaiveDateTime, lat: &str, lon: &str, cog: &str, sog: &str, dpt: &str) -> bool {
    // Date and Time
    window.mv(0, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Time");
    window.attroff(A_REVERSE);
    window.mv(0, 5);
    window.addstr(dt.to_string());
    window.addstr(" UTC");
    // Latitude
    window.mv(2, 0);
    window.clrtoeol();
    window.attron(A_REVERSE);
    window.addstr("Latitude");
    window.attroff(A_REVERSE);
    window.mv(2, 9);
    window.addstr(lat.to_string());
    // Longitude
    window.mv(2, 23);
    window.attron(A_REVERSE);
    window.addstr("Longitude");
    window.attroff(A_REVERSE);
    window.mv(2, 33);
    window.addstr(lon.to_string());
    // COG and SOG
    window.mv(4, 0);
    window.attron(A_REVERSE);
    window.addstr("COG:");
    window.attroff(A_REVERSE);
    window.mv(4, 9);
    window.addstr(cog.to_string());
    window.mv(4, 23);
    window.attron(A_REVERSE);
    window.addstr("SOG:");
    window.attroff(A_REVERSE);
    window.mv(4, 33);
    window.addstr(sog.to_string());
    // Depth
    window.mv(6, 0);
    window.attron(A_REVERSE);
    window.addstr("Depth :");
    window.attroff(A_REVERSE);
    window.mv(6, 9);
    window.addstr(dpt.to_string());
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
