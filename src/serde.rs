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
