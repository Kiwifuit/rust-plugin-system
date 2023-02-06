mod loader;

fn main() {
    let plugin = loader::load_plugin("plugin").unwrap();
}
