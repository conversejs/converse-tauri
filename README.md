Converse Tauri
--------------

This is [Converse.js](https://conversejs.org/) (version 10.1.6) in [Tauri](https://tauri.app) connecting to any XMPP
server through [xmpp-proxy](https://github.com/moparisthebest/xmpp-proxy).

It's the first known client to support [QUIC](https://xmpp.org/extensions/xep-0467.html).

To build install Tauri `cargo install --locked tauri-cli`, then run `cargo tauri build`.

If you want to download binaries head over to [Releases](https://github.com/conversejs/converse-tauri/releases).

####  License
MPLv2      - Converse.js files under dist/ excluding 3rdparty/  
GNU/GPLv3  - dist/3rdparty/libsignal-protocol.min.js  
GNU/AGPLv3 - Rust files under src-tauri/ - Check LICENSE.md for details  
