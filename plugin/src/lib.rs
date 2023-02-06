#[no_mangle]
pub fn start() -> i32 {
    println!("Hello World!");
    0
}

#[no_mangle]
pub fn stop() -> i32 {
    println!("Stopping plugin");
    0
}

#[no_mangle]
pub fn name() -> String {
    String::from("hello-world-plugin")
}

#[no_mangle]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
