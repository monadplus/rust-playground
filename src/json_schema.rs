use jsonschema::JSONSchema;
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct MyStruct {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

#[test]
fn json_schem_test() {
    let schema: RootSchema = schema_for!(MyStruct);
    let schema: Value = serde_json::to_value(&schema).unwrap();
    match JSONSchema::compile(&schema) {
        Err(err) => println!("{err:?}"),
        Ok(schema) => {
            let instance = MyStruct {
                my_int: 1,
                my_bool: true,
                my_nullable_enum: Some(MyEnum::StringNewType("foo".to_string())),
            };
            let instance = serde_json::to_value(&instance).expect("instance cannot be serialized");
            println!("{}", serde_json::to_string_pretty(&instance).unwrap());
            let result = schema.validate(&instance);
            if let Err(errors) = result {
                for error in errors {
                    println!("Validation error: {}", error);
                    println!("Instance path: {}", error.instance_path);
                }
            }
            let bad_instance = json!({
                "my_bool": true,
                "my_int": 1,
                "my_nullable_enum": {
                  "StringNewType": 1
                }
            });
            let result = schema.validate(&bad_instance);
            if let Err(errors) = result {
                for error in errors {
                    println!("Validation error: {}", error);
                    println!("Instance path: {}", error.instance_path);
                }
            }
        }
    }
}
