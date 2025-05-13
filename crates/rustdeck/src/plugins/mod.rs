mod plugin_wrapper;

use std::fs;
use std::path::Path;

pub use plugin_wrapper::Plugin;

pub fn load_plugins_at(path: &Path) -> Result<Vec<Plugin>, Box<dyn std::error::Error>> {
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
        match Plugin::try_load(path) {
            Ok(plugin) => {
                plugins.push(plugin);
            }
            Err(e) => {
                println!("Error loading {:?}:\n -> {}", path, e);
            }
        }
    }

    println!("Loaded plugins ({})", plugins.len());
    Ok(plugins)
}
