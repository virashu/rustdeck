#[derive(Debug)]
pub enum PluginLoadError {
    NotALibrary(libloading::Error),
    GenericLibError(libloading::Error),
    SymbolError(std::str::Utf8Error),
}

impl From<libloading::Error> for PluginLoadError {
    fn from(value: libloading::Error) -> Self {
        match value {
            libloading::Error::LoadLibraryExW { ref source } => {
                let err_code: i32 = format!("{:?}", source)
                    .split_once(",")
                    .unwrap()
                    .0
                    .strip_prefix("Os { code: ")
                    .unwrap()
                    .parse()
                    .unwrap();

                match err_code {
                    193 => Self::NotALibrary(value),
                    _ => Self::GenericLibError(value),
                }
            }
            _ => Self::GenericLibError(value),
        }
    }
}

impl From<std::str::Utf8Error> for PluginLoadError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::SymbolError(value)
    }
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginLoadError::NotALibrary(e) => write!(f, "Not a library: {}", e),
            PluginLoadError::GenericLibError(e) => write!(f, "Error loading library: {}", e),
            PluginLoadError::SymbolError(e) => write!(f, "Symbol error: {}", e),
        }
    }
}

impl std::error::Error for PluginLoadError {}
