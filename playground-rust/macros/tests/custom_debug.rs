#![allow(dead_code)]

use macros::CustomDebug;

#[test]
fn test_parse() {
    #[derive(CustomDebug)]
    pub struct Field {
        name: &'static str,
        bitmask: u16,
    }
}

#[test]
fn test_simple() {
    #[derive(CustomDebug)]
    pub struct Field {
        name: &'static str,
        bitmask: u8,
    }

    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);

    assert!(debug.starts_with(r#"Field { name: "F","#));
}
