use libloading::Library;
use std::ffi::CString;

type StartPlugin = fn() -> i32;
type StopPlugin = fn() -> i32;
type PluginName = fn() -> CString;
type PluginVersion = fn() -> CString;

#[derive(Debug)]
pub enum PluginLoadError {
    LibraryLoadError(libloading::Error),
    SymbolLoadError(libloading::Error),
    StringParseError(std::str::Utf8Error),
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
        }?()
        .to_str()
        .map_err(|e| PluginLoadError::StringParseError(e))?
        .to_string();

        let version = unsafe {
            lib.get::<PluginVersion>(b"version")
                .map_err(|e| PluginLoadError::SymbolLoadError(e))
        }?()
        .to_str()
        .map_err(|e| PluginLoadError::StringParseError(e))?
        .to_string();

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

pub fn load_plugin(path: &str) -> Result<Plugin, PluginLoadError> {
    let name = get_lib_name(path);

    Plugin::new(&name)
}
