//! all the stuff that create a global instance of viperus
//! 
//! the instance is "lazy_static" and proteced by a mutex

use super::*;
use std::sync::Mutex;

lazy_static! {
    /// the global instance
    static ref VIPERUS: Mutex::<Viperus<'static>> = { Mutex::new(Viperus::new()) };
}

/// load_file load a config file in the global instance
pub fn load_file(name: &str, format: Format) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_file(name, format)
}

/// load_adapter ask the adapter to parse her data and merges result map in the internal configurtion map of global instance
pub fn load_adapter(adt: &mut dyn adapter::ConfigAdapter) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_adapter(adt)
}

/// add an override value to the cofiguration
///
/// key is structured in components separated by a "."
pub fn add<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add(key, value)
}

/// get a configuration value of type T from global configuration in this order
/// * overrided key
/// * clap parameters
/// * config adapter sourced values
/// * default value
pub fn get<'a, 'b, T>(key: &'a str) -> Option<T>
where
    map::ViperusValue: From<T>,
    &'b map::ViperusValue: Into<T>,
    map::ViperusValue: Into<T>,
    T: FromStr,
{
    let v = VIPERUS.lock().unwrap();
    v.get(key)
}

/// add an default value to the global cofiguration
///
/// key is structured in components separated by a "."
pub fn add_default<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add_default(key, value)
}

///load_clap  brings in  the clap magic
pub fn load_clap(matches: clap::ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_clap(matches)
}

/// bond a clap argsument to a config key
pub fn bond_clap(src: &str, dst: &str) -> Option<String> {
    VIPERUS.lock().unwrap().bond_clap(src, dst)
}

/// reload the configuration files
pub fn reload() -> Result<(),Box<dyn Error>> {
    VIPERUS.lock().unwrap().reload()
}

 