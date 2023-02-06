# :electric_plug: Simple Rust Plugin System
This repository provides a simple method to load plugins in Rust :crab: with ease. The plugin runner uses [`libloading`](https://crates.io/crates/libloading) to load the plugins, and [`cfg-if`](https://crates.io/crates/cfg-if) to do miscellaneous business logic

If you want to try using this, you must provide these three functions (use `#[no-mangle]` on rust). You can implement these in either Rust or C as long as your output is a dynamic library `.so`/`.dll`

## License
See [LICENSE](./LICENSE)