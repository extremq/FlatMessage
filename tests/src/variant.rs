use flat_message::*;

#[test]
fn check_simple_serde() {

    #[derive(FlatMessageVariant, Debug, PartialEq, Eq)]
    enum Test {
        Byte(u8),
        DWord(u32),
        String(String),
        Vector(Vec<u128>),
        //Reject(u8, String),
        SimpleVariant,
    }
    
}