use serde::{Deserialize, Serialize};

/// This is what we're going to decode into.
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    plain: MyEnum,
    plain_table: MyEnum,
    tuple: MyEnum,
    #[serde(rename = "struct")]
    structv: MyEnum,
    newtype: MyEnum,
    my_enum: Vec<MyEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum MyEnum {
    Plain,
    Tuple(i64, bool),
    NewType(String),
    Struct { value: i64 },
}

fn main() {
    let config = Config {
        plain: MyEnum::Plain,
        plain_table: MyEnum::Plain,
        tuple: MyEnum::Tuple(0, true),
        structv: MyEnum::Struct { value: 1 },
        newtype: MyEnum::NewType("Hello".into()),
        my_enum: vec![
            MyEnum::Plain,
            MyEnum::Tuple(0, true),
            MyEnum::Struct { value: 1 },
            MyEnum::NewType("Hello".into()),
        ],
    };
    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("{}", toml_str);
}
