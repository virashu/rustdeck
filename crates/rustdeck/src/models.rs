use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PluginActionsUngroupedData {
    /// Action id
    pub id: String,
    /// Action display name
    pub name: String,
    /// Action description
    pub description: String,
}

#[derive(Serialize, Clone)]
pub struct PluginActionsGroupedData {
    /// Plugin id
    pub id: String,
    /// Plugin display name
    pub name: String,
    /// Actions of plugin
    pub actions: Vec<PluginActionsUngroupedData>,
}

#[derive(Serialize, Clone)]
pub struct PluginVariablesUngroupedData {
    /// Variable ID
    pub id: String,
    /// Variable description
    pub description: String,
}

#[derive(Serialize, Clone)]
pub struct PluginVariablesGroupedData {
    /// Plugin id
    pub id: String,
    /// Plugin display name
    pub name: String,
    /// Plugin variables
    pub variables: Vec<PluginVariablesUngroupedData>,
}

#[derive(Serialize, Clone)]
pub struct PluginData {
    /// Plugin id
    pub id: String,
    /// Plugin display name
    pub name: String,
    /// Plugin variables
    pub variables: Vec<PluginVariablesUngroupedData>,
    /// Actions of plugin
    pub actions: Vec<PluginActionsUngroupedData>,
}
