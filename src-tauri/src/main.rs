// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod capture;

fn main() {
    
    capture::sip();
    linux_wayland_gif_screenshotter_lib::run();
    capture::sip();
}
