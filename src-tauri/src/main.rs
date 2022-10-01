#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
