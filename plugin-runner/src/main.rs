mod loader;

fn main() {
    let plugin = loader::load_plugin("plugin").unwrap();

    println!(
        "Plugin {} v{} initialized",
        plugin.get_name(),
        plugin.get_version()
    );
    plugin.start();
    plugin.stop();
}
