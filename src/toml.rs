#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

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

    #[test]
    fn toml_test() {
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
        let expected = r#"[plain]
type = "Plain"

[plain_table]
type = "Plain"

[tuple]
type = "Tuple"
value = [
    0,
    true,
]

[struct]
type = "Struct"

[struct.value]
value = 1

[newtype]
type = "NewType"
value = "Hello"

[[my_enum]]
type = "Plain"

[[my_enum]]
type = "Tuple"
value = [
    0,
    true,
]

[[my_enum]]
type = "Struct"

[my_enum.value]
value = 1

[[my_enum]]
type = "NewType"
value = "Hello"
"#;

        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert_eq!(toml_str, expected);
    }
}
