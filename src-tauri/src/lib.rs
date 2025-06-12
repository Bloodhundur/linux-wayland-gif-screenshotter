// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_server::protocol::wl_compositor;
use wayland_server::protocol::wl_pointer;
use wayland_client::Connection;
use wayland_client::protocol::wl_compositor::WlCompositor;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

