use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::RwLock;

use crate::models::{
    PluginActionsGroupedData, PluginActionsUngroupedData, PluginVariablesUngroupedData,
};

use super::{Plugin, load_plugins_at};

pub struct PluginStore {
    plugins: HashMap<String, RwLock<Plugin>>,
}

impl PluginStore {
    pub fn new<S>(path: S) -> Result<Self, io::Error>
    where
        S: AsRef<str>,
    {
        let plugins = load_plugins_at(Path::new(path.as_ref()))?;
        let plugins = plugins
            .into_iter()
            .map(|p| (p.id.clone(), RwLock::new(p)))
            .collect();

        Ok(Self { plugins })
    }

    pub fn update_all(&self) {
        self.plugins
            .values()
            .for_each(|p| p.write().unwrap().update());
    }

    pub fn try_resolve_variable<S>(&self, id: S) -> Result<String, String>
    where
        S: AsRef<str>,
    {
        let (plug_id, i) = id.as_ref().split_once('.').ok_or("Wrong variable format")?;
        let plugin = self
            .plugins
            .get(plug_id)
            .ok_or_else(|| format!("Cannot find plugin: `{plug_id}`"))?
            .read()
            .unwrap();

        if !plugin.variables.iter().any(|v| v.id == i) {
            return Err(format!(
                "Plugin `{plug_id}` does not provide variable `{i}`"
            ));
        }

        Ok(plugin.get_variable(i))
    }

    pub fn render_variable<S>(&self, id: S) -> String
    where
        S: AsRef<str>,
    {
        match self.try_resolve_variable(id) {
            Err(s) | Ok(s) => s,
        }
    }

    pub fn try_run_action<S>(&self, id: S) -> Result<(), String>
    where
        S: AsRef<str>,
    {
        let (plug_id, i) = id
            .as_ref()
            .split_once('.')
            .ok_or_else(|| format!("Wrong action format: `{}`", id.as_ref()))?;

        {
            let plugin = self
                .plugins
                .get(plug_id)
                .ok_or_else(|| format!("Cannot find plugin: `{plug_id}`"))?
                .read()
                .unwrap();

            if !plugin.actions.iter().any(|v| v.id == i) {
                return Err(format!("Plugin `{plug_id}` does not provide action `{i}`"));
            }

            plugin.run_action(i.to_string());
        }

        Ok(())
    }

    pub fn get_all_variables(&self) -> Vec<PluginVariablesUngroupedData> {
        let mut vars = Vec::new();

        for (plugin_id, plugin) in &self.plugins {
            let vars_internal = plugin.read().unwrap().variables.clone();

            for var in vars_internal {
                let var_id = format!("{plugin_id}.{}", var.id);
                // self.render_variable(var_id)
                vars.push(PluginVariablesUngroupedData {
                    id: var_id,
                    description: var.description,
                });
            }
        }

        vars
    }

    pub fn get_all_actions_names(&self) -> Vec<PluginActionsUngroupedData> {
        let mut acts = Vec::new();

        for (plugin_id, plugin) in &self.plugins {
            let lock = plugin.read().unwrap();
            for act in &lock.actions {
                let act_id = format!("{plugin_id}.{}", act.id);
                acts.push(PluginActionsUngroupedData {
                    id: act_id,
                    name: act.name.clone(),
                    description: act.description.clone(),
                });
            }
        }

        acts
    }

    pub fn get_all_actions(&self) -> Vec<PluginActionsGroupedData> {
        let mut acts = Vec::new();

        for (plugin_id, plugin) in &self.plugins {
            let lock = plugin.read().unwrap();
            acts.push(PluginActionsGroupedData {
                id: plugin_id.clone(),
                name: lock.name.clone(),
                actions: lock
                    .actions
                    .clone()
                    .into_iter()
                    .map(|a| PluginActionsUngroupedData {
                        id: a.id,
                        name: a.name,
                        description: a.description,
                    })
                    .collect(),
            });
        }

        acts
    }
}
