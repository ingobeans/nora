# nora
<img width="280" height="280" alt="image" src="https://github.com/user-attachments/assets/c5b79669-4c00-432f-a459-8debb87c2d74" />
<img width="457" height="280" alt="image" src="https://github.com/user-attachments/assets/0377b8d7-6582-49a0-a810-ac3d98989b71" />

this project is a really unfinished platformer written in rust! its still very much a WIP, but right now theres 7 levels you can beat.

## controls!!

* WASD to move.
* SHIFT to slide (important!)
* SPACE to jump.

(thats it)

## build

you need rust and cargo to build. to build locally its just `cargo run`

for a web build with `basic-http-server`, do: `cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/nora.wasm web/ && basic-http-server web/`
