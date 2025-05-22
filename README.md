<h2 align="center">

  <a href="https://conversejs.org" target="_blank" rel="noopener">
    <img alt="Converse.js" src="https://github.com/conversejs/converse.js/blob/master/logo/readme.png" width="480">
  </a>
</h2>


# Converse Tauri

Converse Tauri is an XMPP chat client that integrates [Converse](https://conversejs.org/) with [Tauri](https://tauri.app) and [xmpp-proxy](https://github.com/moparisthebest/xmpp-proxy).

`xmpp-proxy` lets you connect to any XMPP server, even if it doesn't support
[BOSH](https://xmpp.org/extensions/xep-0124.html) or websocket, which are
requirements when Converse is running in a normal web browser.
It is also the first known client to support [QUIC](https://xmpp.org/extensions/xep-0467.html).

## Installation

If you want to download binaries head over to [Releases](https://github.com/conversejs/converse-tauri/releases).

To build and run the application, follow these steps:

1. **Install Rust**: Ensure you have Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).

2. **Install Tauri CLI**: Run the following command to install the Tauri CLI:
   ```bash
   cargo install --locked tauri-cli
   ```

3. **Clone the Repository**: Clone this repository to your local machine:
   ```bash
   git clone https://github.com/conversejs/converse-tauri.git
   cd converse-tauri
   ```

4. **Build the Application**: Run the following command to build the application:
   ```bash
   cargo tauri build
   ```

5. **Run the Application**: You can run the application in development mode using:
   ```bash
   cargo tauri dev
   ```

## License

This project is licensed under the following licenses:

- **MPLv2**: Converse.js files under `dist/` excluding `3rdparty/`
- **GNU/GPLv3**: `dist/3rdparty/libsignal-protocol.min.js`
- **GNU/AGPLv3**: Rust files under `src-tauri/` - Check `LICENSE.md` for details.
