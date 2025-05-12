use std::error::Error;
use std::fs;
use std::path::Path;

use common::PluginTrait;
use plugin_wrapper::RsPluginWrapper;

mod error;
mod plugin_wrapper;

fn load_plugins_at(path: &Path) -> Result<Vec<Box<dyn PluginTrait>>, Box<dyn Error>> {
    let mut plugins: Vec<Box<dyn PluginTrait>> = Vec::new();

    let dir = fs::read_dir(path)?;
    let entries = dir.flatten();
    let paths = entries.map(|e| e.path()).collect::<Vec<_>>();
    let libs = &paths
        .iter()
        .filter(|p| p.is_file())
        .filter(|p| p.to_str().unwrap().ends_with(".dll"))
        .collect::<Vec<_>>();

    for path in libs {
        match RsPluginWrapper::try_load(path) {
            Ok(plugin) => {
                plugins.push(Box::new(plugin));
            }
            Err(e) => {
                println!("Error loading {:?}:\n -> {}", path, e);
            }
        }
    }

    println!("Loaded plugins ({})", plugins.len());
    Ok(plugins)
}

fn main() {
    let plugins = load_plugins_at(Path::new("./plugins")).unwrap();

    for (i, plugin) in plugins.iter().enumerate() {
        print!("{}) ", i + 1);
        println!("{}", plugin.get_name());
    }

    println!("OK");
}
