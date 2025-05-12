use std::error::Error;
use std::fs;
use std::path::Path;

use plugin_wrapper::PluginWrapper;

mod plugin_wrapper;
mod error;

fn load_plugins_at(path: &Path) -> Result<Vec<PluginWrapper>, Box<dyn Error>> {
    let mut plugins = Vec::new();

    let dir = fs::read_dir(path)?;
    let entries = dir.flatten();
    let paths = entries.map(|e| e.path()).collect::<Vec<_>>();
    let libs = &paths
        .iter()
        .filter(|p| p.is_file())
        .filter(|p| p.to_str().unwrap().ends_with(".dll"))
        .collect::<Vec<_>>();

    for path in libs {
        match PluginWrapper::try_load(path) {
            Ok(plugin) => {
                plugins.push(plugin);
            }
            Err(_) => {
                println!("Error loading {:?}", path);
                // report_error(e);
            }
        }
    }

    println!("Loaded all plugins.");
    Ok(plugins)
}

fn main() {
    let plugins = load_plugins_at(Path::new("./plugins")).unwrap();

    println!("Trying to read all plugins' names outside loader");

    for (i, plugin) in plugins.iter().enumerate() {
        println!("[{}] =====", i + 1);
        println!("Name: {}", plugin.get_name());
    }
    println!("=========");

    println!("OK");
}
