# Stop Yourself: A game about self improvement
## Core Mechanic
- Platform across an obstacle course to reach the level's goal, a flag!
- Done that? Add an obstacle to Stop Yourself (i.e. the replay of your previous win)
- And beat your level again!
Get the highest possible score by beating your level as many times as you can.

## Running
The game uses the Bevy Game Engine. To run, clone the repository and simply `cargo run`.

### Compiling to wasm
Follow https://bevy-cheatbook.github.io/platforms/wasm.html
### Compiling to native executable
Remove the `dynamic_linking` flag from the bevy features, and make sure to pass the `--release` flag to cargo.
Recommended release profile:
```toml
[profile.release]
codegen-units=1
lto="fat"
opt-level =3
incremental=false
```
which is pretty much the most optimized build you can get.

