#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use std::sync::Arc;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::net::TcpListener;
use xmpp_proxy::{
    common::{certs_key::CertsKey, outgoing::OutgoingConfig},
    outgoing::spawn_outgoing_listener,
};

static PROXY_PORT: OnceCell<Result<u16, &'static str>> = OnceCell::new();

#[tauri::command]
fn proxy_port() -> Result<u16, &'static str> {
    println!("returning proxy_port");
    *PROXY_PORT.wait()
}

async fn start_proxy() -> Result<u16, std::io::Error> {
    let outgoing_cfg = OutgoingConfig {
        // limit incoming stanzas to this many bytes, default to ejabberd's default
        // https://github.com/processone/ejabberd/blob/master/ejabberd.yml.example#L32
        // xmpp-proxy will use this many bytes + 16k per connection
        max_stanza_size_bytes: 262_144,
        certs_key: Arc::new(CertsKey {}),
    };
    // try to listen to a specific port (10032), but otherwise listen on any available
    let listener = match TcpListener::bind("127.0.0.1:10032").await {
        Ok(listener) => listener,
        Err(_) => TcpListener::bind("127.0.0.1:0").await?,
    };
    let port = listener.local_addr()?.port();
    spawn_outgoing_listener(listener, outgoing_cfg);
    println!("started proxy at port: {port}");
    Ok(port)
}

fn main() {
    tauri::async_runtime::spawn(async {
        PROXY_PORT
            .set(
                start_proxy()
                    .await
                    .map_err(|e| Box::leak(Box::new(format!("{:?}", e))).as_str()),
            )
            .expect("this is the only place that calls set");
    });
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
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
        .invoke_handler(tauri::generate_handler![proxy_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
