use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    amount: Decimal,
    fee: Decimal,
}

#[test]
fn decimal_test() {
    let dec = Decimal::new(100_125_00, 5);
    assert_eq!(dec.to_string(), "100.12500");
    let dec_norm = dec.normalize();
    assert_eq!(dec_norm.to_string(), "100.125");
    let p = Decimal::from_scientific("3.16912650057e+25").unwrap();
}

#[test]
fn decimal_serialization_test() {
    let item = Item {
        name: "Apple".to_string(),
        amount: dec!(125_000),
        fee: dec!(0.0000000000000000005),
    };
    assert_eq!(
        serde_json::to_value(&item).unwrap(),
        json!({
            "name": "Apple",
            "amount": 125000.0,
            "fee": 0.0000000000000000005,
        })
    );
}
