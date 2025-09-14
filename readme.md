# nora
<img width="148" height="160" alt="image" src="https://github.com/user-attachments/assets/c5b79669-4c00-432f-a459-8debb87c2d74" />

this project is a really unfinished platformer written in rust! its still very much a WIP, but right now theres 7 levels you can beat.

## build

you need rust and cargo to build. to build locally its just `cargo run`

for a web build with `basic-http-server`, do: `cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/nora.wasm web/ && basic-http-server web/`