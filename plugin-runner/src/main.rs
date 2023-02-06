mod loader;

fn main() {
    let plugins =
        vec!["/home/misery/Documents/Code/Rust/simple-plugin-system/plugin-runner/hello-world"];
    let (manager, errors) = loader::PluginManager::new(plugins);

    if !errors.is_empty() {
        eprintln!("Error occured while loading plugins:");

        for (plugin, err) in errors {
            eprintln!("[{}] {}", plugin, err);
        }
    }

    manager.start_all().iter().for_each(|(name, code)| {
        println!("[{}] finished initializing with {:x}", name, code);
    });
    manager.stop_all().iter().for_each(|(name, code)| {
        println!("[{}] finished de-initializing with {:x}", name, code);
    });
}
