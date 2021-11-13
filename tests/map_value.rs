#[cfg(test)]
mod tests {
    use viperus::ViperusValue;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_value2bool() {
        init();

        let value = ViperusValue::Empty;
        let _b: bool = value.into();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_ref_value2bool() {
        init();

        let value = &ViperusValue::Empty;
        let _b: bool = value.into();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_value2i32() {
        init();

        let value = &ViperusValue::Empty;
        let _b: i32 = value.into();
    }
    #[test]
    fn valid_cast_value2bool() {
        init();

        let value = ViperusValue::BOOL(true);
        let b: bool = value.into();
        assert!(b);
    }

    #[test]
    fn valid_cast_str2value() {
        init();

        let value = ViperusValue::from("hello world!");
        match value {
            ViperusValue::Str(s) => assert_eq!(s, "hello world!"),
            _ => panic!("something very wrong"),
        }

        let ref_value = ViperusValue::from(&("hello world!".to_owned()));
        match ref_value {
            ViperusValue::Str(s) => assert_eq!(s, "hello world!"),
            _ => panic!("something very wrong"),
        }
    }

    #[test]
    #[should_panic]
    fn invalid_cast_value2string() {
        init();

        let value = &ViperusValue::Empty;
        let _b: String = value.into();
    }
}
