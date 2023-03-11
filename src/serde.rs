use serde::{Deserialize, Deserializer};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Config {
    caching: BoolOrTime,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum BoolOrTime {
    #[serde(deserialize_with = "deserialize_disabled_field_2")]
    Disabled,
    #[serde(with = "humantime_serde")]
    Duration(Duration),
}

#[allow(dead_code)]
fn deserialize_disabled_field<'de, D>(de: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(de)?.as_str() {
        "disabled" | "false" => Ok(()),
        unexpected => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(unexpected),
            &"disabled or false",
        )),
    }
}

fn deserialize_disabled_field_2<'de, D>(de: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    enum Helper {
        #[serde(rename = "disabled")]
        Variant,
    }

    Helper::deserialize(de).map(|_| ())
}

#[test]
fn serde_untagged() {
    let obj = serde_json::json!({
        "caching": "disabled"
    });
    assert_eq!(
        Config {
            caching: BoolOrTime::Disabled
        },
        serde_json::from_value(obj).unwrap()
    );

    let obj = serde_json::json!({
        "caching": "10 s"
    });
    assert_eq!(
        Config {
            caching: BoolOrTime::Duration(Duration::from_secs(10))
        },
        serde_json::from_value(obj).unwrap()
    );
}

#[test]
fn generic_deserializer_test() {
    use serde::{
        de::{Deserializer, Error as DeError},
        forward_to_deserialize_any, Deserialize,
    };

    pub struct Wrapper<D> {
        de: D,
        count: usize,
    }

    impl<D> Wrapper<D> {
        pub fn new(de: D) -> Self {
            Wrapper { de, count: 0 }
        }
    }

    impl<'de, 'a, D> Deserializer<'de> for &'a mut Wrapper<D>
    where
        &'a mut D: Deserializer<'de>,
    {
        type Error = <&'a mut D as Deserializer<'de>>::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.count = self.count + 1;
            self.de
                .deserialize_any(visitor)
                .map_err(|e| DeError::custom(format!("deserialize_any: {}", e)))
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
        }
    }

    #[derive(Debug, Deserialize)]
    struct MyType {
        foo: String,
        qux: u64,
    }

    let json = r#"{"foo": "bar", "qux": 42}"#;
    let mut json_des = serde_json::Deserializer::new(serde_json::de::StrRead::new(json));
    let mut d = Wrapper::new(json_des);
    let thing = MyType::deserialize(&mut d);
    dbg!(thing);
}
