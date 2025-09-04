use flat_message::*;


macro_rules! serde {
    ($obj:expr, $T:ty, $s:expr) => {
        {
            $obj.serialize_to(&mut $s, Config::default()).unwrap();
            let r = <$T>::deserialize_from(&$s).unwrap();
            r
        }
    };
}

#[test]
fn check_u32_default() {
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true)]
        b: u32,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: 2 }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, 0); // default for u32 is 0
}

#[test]
fn check_u32_custom_default() {
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true, default = 41)]
        b: u32,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: 2 }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, 41); // custom default for u32 set to 41 for this field
}

#[test]
fn check_u32_custom_default_with_constant() {
    const MY_CONST: u32 = 10;
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true, default = MY_CONST)]
        b: u32,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: 2 }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, 10); // custom default for u32 set to 10 for this field
}

#[test]
fn check_u32_custom_default_with_expression() {
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true, default = "1+2+3")]
        b: u32,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: 2 }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, 6); // custom default for u32 set to 6 for this field
}

#[test]
fn check_option_u32_default() {
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true)]
        b: Option<u32>,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: Some(2)}, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, None); // custom default for Option<u32> is None
}


#[test]
fn check_option_u32_custom_default() {
    #[derive(FlatMessage)]
    struct Test {
        a: u8,
        #[flat_message_item(skip : true, default = "41")]
        b: Option<u32>,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: Some(2)}, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, Some(41)); // custom default for Option<u32> is Some(41) (41 should be provided as a string "41" in order of it to be converted into Some(41))
}


#[test]
fn check_str_default() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true)]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, ""); // default for &str is ""
}

#[test]
fn check_str_custom_default() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = "hello")]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, "hello"); // custom default for &str is "hello"
}

#[test]
fn check_str_custom_default_with_constant() {
    const MY_CONST: &str = "hello";
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = MY_CONST)]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, "hello"); // custom default for &str is "hello"
}

#[test]
fn check_str_custom_default_with_constant_using_raw_string() {
    const MY_CONST: &str = "hello";
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = r#"MY_CONST"#)]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, "hello"); // custom default for &str is "hello"
}

#[test]
fn check_str_custom_default_with_default_value_using_raw_string() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = r#""hello""#)]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, "hello"); // custom default for &str is "hello"
}

#[test]
fn check_str_custom_default_with_default_value_from_function() {
    fn foo() -> &'static str {
        "hello"
    }
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = r#"foo()"#)]
        b: &'a str,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: "xyz" }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, "hello"); // custom default for &str is "hello"
}

#[test]
fn check_option_str_default() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true)]
        b: Option<&'a str>,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: Some("xyz") }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, None); // custom default for Option<&str> is None
}

#[test]
fn check_option_str_custom_default() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = "hello")]
        b: Option<&'a str>,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: Some("xyz") }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, Some("hello")); // custom default for Option<&str> is Some("hello")
}

#[test]
fn check_option_str_custom_default_2() {
    #[derive(FlatMessage)]
    struct Test<'a> {
        a: u8,
        #[flat_message_item(skip : true, default = "hello")]
        b: Option<&'a str>,
    }
    let mut s = Storage::default();
    let r = serde!(Test { a: 1, b: Some("xyz") }, Test, s);
    assert_eq!(r.a, 1);
    assert_eq!(r.b, Some("hello")); // custom default for Option<&str> is Some("hello")
}