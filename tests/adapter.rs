#[cfg(test)]
mod tests {
    pub use viperus::adapter::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[cfg(feature = "fmt-json")]
    fn adapter_json_load() {
        init();

        let mut a = JsonAdapter::new();
        a.load(&mut "{ \"json\": true }".as_bytes()).unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("json").unwrap();
        assert!(jtrue, "{}", true);
    }

    #[test]
    #[cfg(feature = "fmt-yaml")]
    fn adapter_yaml_load() {
        init();

        let mut a = YamlAdapter::new();
        a.load(&mut "yaml: true\n".as_bytes()).unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("yaml").unwrap();
        assert!(jtrue,"{}", true);
    }

    #[test]
    #[cfg(feature = "fmt-toml")]
    fn adapter_toml_load() {
        init();

        let mut a = TomlAdapter::new();
        a.load(&mut "[level1]\nkey1=true\nkeyi32=42\nkey=\"hello world!\"\n".as_bytes())
            .unwrap();
        a.parse().unwrap();

        let map = a.get_map();
        let jtrue = map.get::<bool>("level1.key1").unwrap();
        assert!(jtrue, "{}", true);

        let ji32 = map.get::<i32>("level1.keyi32").unwrap();
        assert_eq!(ji32, 42);

        let jstr = map.get::<String>("level1.key").unwrap();
        assert_eq!(jstr, "hello world!");
    }
}
