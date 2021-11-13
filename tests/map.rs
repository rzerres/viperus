#[cfg(test)]
mod tests {
    use viperus::ViperusValue;
    use viperus::map::Map;

    #[test]
    fn test_map_add_get() {
        let mut m = Map::new();
        let mv0 = m.add_value("test.value", ViperusValue::I32(10));
        assert_eq!(None, mv0);
        let mv1 = m.get_value("test.value").unwrap();
        if let ViperusValue::I32(v1) = mv1 {
            assert_eq!(10, *v1);
        }
    }

    #[test]
    fn test_map_get_32() {
        let mut m = Map::default();
        m.add_value("test.value2", ViperusValue::from("none"));

        let mv0 = m.add_value("test.value", ViperusValue::from(42));
        assert_eq!(None, mv0);

        let _a1 = m.add::<i32>("test.value_i32", 314).unwrap_or_default();
        let _a2 = m.add::<i32>("test.value_i32", 314).unwrap_or_default();

        let v1 = m.get::<i32>("test.value").unwrap();
        assert_eq!(42, v1);

        let v1_i32 = m.get::<i32>("test.value_i32").unwrap();
        assert_eq!(314, v1_i32);

        let v1_str = m.get::<String>("test.value2").unwrap();
        assert_eq!("none", v1_str);
    }
    #[test]
    #[should_panic]
    fn invalid_map_get_32() {
        let mut m = Map::default();
        m.add_value("test.value2", ViperusValue::from("none"));

        assert!(m.get::<i32>("test.value2").is_none())
    }
}
