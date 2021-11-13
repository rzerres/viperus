#[cfg(feature = "fmt-env")]
mod adapter_env;

#[cfg(feature = "fmt-json")]
mod adapter_json;

#[cfg(feature = "fmt-javaproperties")]
mod adapter_prop;

#[cfg(feature = "fmt-toml")]
mod adapter_toml;

#[cfg(feature = "fmt-yaml")]
mod adapter_yaml;


/// Feature `fmt-env`: Provides the handling Environment parameters.
#[cfg(feature = "fmt-env")]
pub use adapter_env::*;

/// Feature `fmt-json`: Provides the handling of Json files.
#[cfg(feature = "fmt-json")]
pub use adapter_json::*;

/// Feature `fmt-javaproperties`: Provides the handling of Java propertie files.
#[cfg(feature = "fmt-javaproperties")]
pub use adapter_prop::*;

/// Feature `fmt-toml`: Provides the handling of Toml files.
#[cfg(feature = "fmt-toml")]
pub use adapter_toml::*;

/// Feature `fmt-yaml`: Provides the handling of Yaml files.
#[cfg(feature = "fmt-yaml")]
pub use adapter_yaml::*;

/// Returns the result value when consuming an adapter.
pub type AdapterResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Mediates varius config formats and Viperus
pub trait ConfigAdapter {
    /// Parse the internal representation of the config parameters.
    fn parse(&mut self) -> AdapterResult<()>;
    /// Returns a key value map representation of the actual config.
    fn get_map(&self) -> crate::map::Map;
}
