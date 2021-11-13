//! A reference counted global `VIPERUS` instance.
//!
//! The instance is "lazy_static" and proteced by a mutex.

use super::*;
use std::sync::mpsc::channel;

#[cfg(feature = "watch")]
use notify::Watcher;
use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "global")]
lazy_static! {
    /// A mutex guarded reference countner to a static `VIPERUS` instance
    static ref VIPERUS: Arc::<Mutex::<Viperus<'static>>> = Arc::new(Mutex::new(Viperus::new()));
}

/// Watch the config files and autoreload in case of change
///
/// The function starts a separate thread.
/// TODO ad an unwatch_all() function;
#[cfg(feature = "watch")]
pub fn watch_all() -> Result<(), Box<dyn Error>> {
    let lf = VIPERUS.lock().unwrap().loaded_file_names();

    let vip = VIPERUS.clone();

    std::thread::spawn(move || {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        let mut watcher: notify::RecommendedWatcher =
            notify::Watcher::new(tx, Duration::from_secs(2)).unwrap();

        // Add a watch path. All files and directories are checked.
        for f in lf {
            watcher
                .watch(f, notify::RecursiveMode::NonRecursive)
                .unwrap();
        }

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            match rx.recv() {
                Ok(event) => {
                    info!("watch {:?}", event);
                    vip.lock().unwrap().reload().unwrap();
                }
                Err(e) => error!("watch error: {:?}", e),
            }
        }
    });

    Ok(())
}

/// Loads a config file into the global instance.
pub fn load_file(name: &str, format: Format) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_file(name, format)
}

/// Ask the adapter to parse its data and merge the result map into
/// the internal configuration map of the global instance.
pub fn load_adapter(adt: &mut dyn adapter::ConfigAdapter) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_adapter(adt)
}

/// Add an override value to the configuration map.
///
/// The keys are structured as components, separated by a "."
pub fn add<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add(key, value)
}

/// Get a configuration value of type `T` from the global configuration.
///
/// Parsing will respect the following order:
///
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
    T: Clone,
{
    VIPERUS.lock().unwrap().get(key)
}

/// Add an default value to the global configuration.
///
/// The keys are structured as components, separated by a "."
pub fn add_default<T>(key: &str, value: T) -> Option<T>
where
    map::ViperusValue: From<T>,
    map::ViperusValue: Into<T>,
{
    VIPERUS.lock().unwrap().add_default(key, value)
}

/// Incoprorate the clap magic.
///
/// This requires the opt-in of feature `fmt-clap` thas consumes the `clap` crate.
#[cfg(feature = "fmt-clap")]
pub fn load_clap(matches: clap::ArgMatches<'static>) -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().load_clap(matches)
}

/// Bond a clap argument to a config key.
#[cfg(feature = "fmt-clap")]
pub fn bond_clap(src: &str, dst: &str) -> Option<String> {
    VIPERUS.lock().unwrap().bond_clap(src, dst)
}

/// Reload the configuration files.
pub fn reload() -> Result<(), Box<dyn Error>> {
    VIPERUS.lock().unwrap().reload()
}

/// Cache the query results (gain a x4 speedup for small config files).
///
/// since v0.1.9: returns the previus state , useful for test setups.
#[cfg(feature = "cache")]
pub fn cache(enable: bool) -> bool {
    VIPERUS.lock().unwrap().cache(enable)
}

/// Enables the automatic probe of an environment variable.
///
/// `Get`-calls will issue the reload of the given environment variable that matches the uppercase key name.
/// If `env_prefix` is enables, the given prefix will be prepended to the key name.
pub fn automatic_env(enable: bool) {
    VIPERUS.lock().unwrap().automatic_env(enable)
}

/// Prepend the `prefix` to given key name when probing environment variables.
pub fn set_env_prefix(prefix: &str) {
    VIPERUS.lock().unwrap().set_env_prefix(prefix)
}
