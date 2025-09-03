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