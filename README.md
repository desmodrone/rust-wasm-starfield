# rust-wasm-starfield

A small Macroquad experiment: a browser-based starfield written in Rust and compiled to WebAssembly.

This repo is intentionally simple. It is not using Trunk or a frontend framework. The browser build uses Macroquad's HTML5 loader path with a small `web/` directory containing:

- `index.html`
- `mq_js_bundle.js`
- the compiled `.wasm`

## What It Does

- renders a 3D-like starfield in the browser
- uses a simple projection trick based on `x / z` and `y / z`
- adds a centered `RUST // WASM` overlay as a final polish pass

## Requirements

- Rust and Cargo
- `wasm32-unknown-unknown` target
- `basic-http-server` for local serving

Install the missing pieces with:

```bash
rustup target add wasm32-unknown-unknown
cargo install basic-http-server
```

## Run It

From the repo root:

```bash
chmod +x build.sh
./build.sh
```

Then open:

```text
http://127.0.0.1:4000
```

## Development Loop

This setup does **not** hot-reload.

When you change the Rust code:

1. rerun `./build.sh`
2. refresh the browser

If the local server is already running on port `4000`, the script will rebuild the wasm and tell you to refresh instead of trying to start a second server.

## Project Layout

```text
.
├── Cargo.toml
├── build.sh
├── src/
│   └── main.rs
└── web/
    ├── index.html
    ├── mq_js_bundle.js
    └── rust-wasm-starfield.wasm
```

## Notes

- `mq_js_bundle.js` is the correct browser loader for this setup.
- The filename in `load("...")` inside `web/index.html` must exactly match the wasm file in `web/`.
- `rand` is configured with `default-features = false` and `small_rng` to avoid pulling in browser glue that conflicts with Macroquad's plain HTML5 loader path.

## Common Problems

### Black screen

Check:

- `web/index.html` exists
- `web/mq_js_bundle.js` is a real JS file, not an HTML error page
- `web/rust-wasm-starfield.wasm` exists
- the `load("...")` filename matches the wasm file exactly

### `__wbindgen_placeholder__` errors

If you see console errors involving `__wbindgen_placeholder__`, the wasm was built with a dependency path expecting `wasm-bindgen`, which does not match `mq_js_bundle.js`.

For this repo, that was fixed by keeping the dependencies minimal:

```toml
[dependencies]
macroquad = "0.4"
rand = { version = "0.8", default-features = false, features = ["small_rng"] }
```

## Why This Repo Exists

This project was built alongside a tutorial draft about using Rust + Macroquad + WASM to make something visual quickly. The goal is not a full engine. It is a compact example that is easy to inspect, tweak, and run in the browser.
