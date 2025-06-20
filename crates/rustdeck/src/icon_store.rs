use std::collections::HashMap;

pub struct IconStore {
    icons: HashMap<String, String>,
}

impl IconStore {
    pub const fn from_config(icons: HashMap<String, String>) -> Self {
        Self { icons }
    }

    pub fn to_config(&self) -> HashMap<String, String> {
        self.icons.clone()
    }

    pub fn get_icon<S>(&self, id: S) -> Option<&String>
    where
        S: AsRef<str>,
    {
        self.icons.get(id.as_ref())
    }

    pub fn add_icon(&mut self) {}

    pub fn keys(&self) -> Vec<String> {
        self.icons.keys().cloned().collect()
    }
}
