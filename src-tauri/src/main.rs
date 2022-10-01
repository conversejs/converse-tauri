#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::net::TcpListener;
use xmpp_proxy::{
    common::{certs_key::CertsKey, outgoing::OutgoingConfig},
    outgoing::spawn_outgoing_listener,
};

#[tauri::command]
async fn start_proxy() -> u16 {
    println!("starting proxy");
    let outgoing_cfg = OutgoingConfig {
        // limit incoming stanzas to this many bytes, default to ejabberd's default
        // https://github.com/processone/ejabberd/blob/master/ejabberd.yml.example#L32
        // xmpp-proxy will use this many bytes + 16k per connection
        max_stanza_size_bytes: 262_144,
        certs_key: Arc::new(CertsKey {}),
    };
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    spawn_outgoing_listener(listener, outgoing_cfg);
    println!("started proxy at port: {port}");
    port
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
                // doesn't seem to work
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
                // doesn't seem to work
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
                // doesn't seem to work
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![start_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
