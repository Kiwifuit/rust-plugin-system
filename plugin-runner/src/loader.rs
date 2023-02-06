use std::collections::HashMap;
use std::fmt::Display;

use libloading::Library;

type StartPlugin = fn() -> i32;
type StopPlugin = fn() -> i32;
type PluginName = fn() -> String;
type PluginVersion = fn() -> String;

#[derive(Debug)]
pub enum PluginLoadError {
    LibraryLoadError(libloading::Error),
    SymbolLoadError(libloading::Error),
}

impl Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PluginLoadError::LibraryLoadError(e) =>
                    format!("error occurred while loading {}", e),
                PluginLoadError::SymbolLoadError(e) =>
                    format!("error occurred while loading a symbol: {}", e),
            }
        )
    }
}

pub struct Plugin {
    name: String,
    version: String,
    library: Library,
}

impl Plugin {
    pub fn new(path: &String) -> Result<Self, PluginLoadError> {
        let lib =
            unsafe { Library::new(path) }.map_err(|e| PluginLoadError::LibraryLoadError(e))?;

        let name = unsafe {
            lib.get::<PluginName>(b"name")
                .map_err(|e| PluginLoadError::SymbolLoadError(e))
        }?();

        let version = unsafe {
            lib.get::<PluginVersion>(b"version")
                .map_err(|e| PluginLoadError::SymbolLoadError(e))
        }?();

        Ok(Self {
            name,
            version,
            library: lib,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn start(&self) -> Result<i32, PluginLoadError> {
        Ok(unsafe { self.library.get::<StartPlugin>(b"start") }
            .map_err(|e| PluginLoadError::SymbolLoadError(e))?(
        ))
    }

    pub fn stop(&self) -> Result<i32, PluginLoadError> {
        Ok(unsafe { self.library.get::<StopPlugin>(b"stop") }
            .map_err(|e| PluginLoadError::SymbolLoadError(e))?(
        ))
    }
}

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn get_lib_name(name: &str) -> String {
            String::from(name.rsplit(".").collect::<Vec<&str>>()[0]) + ".so"
        }
    } else if #[cfg(windows)] {
        fn get_lib_name(name: &str) -> String {
            String::from(name.rsplit(".").collect::<Vec<&str>>()[0]) + ".dll"
        }
    }
}

pub fn load_plugin(name: &str) -> Result<Plugin, PluginLoadError> {
    let name = get_lib_name(name);

    Plugin::new(&name)
}

pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new<'a>(plugins: Vec<&'a str>) -> (Self, Vec<(&'a str, PluginLoadError)>) {
        let mut errors = vec![];
        let mut plugins_loaded = vec![];

        for plugin in plugins {
            match load_plugin(plugin) {
                Ok(p) => plugins_loaded.push(p),
                Err(e) => errors.push((plugin, e)),
            };
        }

        (
            Self {
                plugins: plugins_loaded,
            },
            errors,
        )
    }

    pub fn start_all(&self) -> HashMap<String, i32> {
        let mut returned = HashMap::new();

        for plugin in self.plugins.iter() {
            let name = format!("{} v{}", plugin.get_name(), plugin.get_version());
            let status = plugin.start();

            match status {
                Ok(stat_code) => {
                    returned.insert(name, stat_code);
                }
                Err(err) => {
                    eprintln!("[{}] {}", name, err);
                }
            }
        }

        returned
    }

    pub fn stop_all(&self) -> HashMap<String, i32> {
        let mut returned = HashMap::new();

        for plugin in self.plugins.iter() {
            let name = format!("{} v{}", plugin.get_name(), plugin.get_version());
            let status = plugin.stop();

            match status {
                Ok(stat_code) => {
                    returned.insert(name, stat_code);
                }
                Err(err) => {
                    eprintln!("[{}] {}", name, err);
                }
            }
        }

        returned
    }
}
