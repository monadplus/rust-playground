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
        my_enum: MyEnum,
        my_vec: Vec<MyEnum>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type", content = "value")]
    enum MyEnum {
        Plain,
        Tuple(i64, bool),
        NewType(String),
        Struct { value: i64 },
        Enum(MyEnum2),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum MyEnum2 {
        VariantA(MyEnum3),
        VariantB,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum MyEnum3 {
        VariantA { a: u8 },
    }

    #[test]
    fn yaml_test() {
        let config = Config {
            plain: MyEnum::Plain,
            plain_table: MyEnum::Plain,
            tuple: MyEnum::Tuple(0, true),
            structv: MyEnum::Struct { value: 1 },
            newtype: MyEnum::NewType("Hello".into()),
            my_enum: MyEnum::Enum(MyEnum2::VariantA(MyEnum3::VariantA { a: 0 })),
            my_vec: vec![
                MyEnum::Plain,
                MyEnum::Tuple(0, true),
                MyEnum::Struct { value: 1 },
                MyEnum::NewType("Hello".into()),
            ],
        };
        let expected = indoc::indoc! {r#"
            plain:
              type: Plain
            plain_table:
              type: Plain
            tuple:
              type: Tuple
              value:
              - 0
              - true
            struct:
              type: Struct
              value:
                value: 1
            newtype:
              type: NewType
              value: Hello
            my_enum:
              type: Enum
              value:
                a: 0
            my_vec:
            - type: Plain
            - type: Tuple
              value:
              - 0
              - true
            - type: Struct
              value:
                value: 1
            - type: NewType
              value: Hello
        "#};
        let yaml_str = serde_yaml::to_string(&config).unwrap();
        assert_eq!(yaml_str, expected);
    }
}
