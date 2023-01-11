use prost::{Message, Oneof};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Message)]
pub struct MyStruct {
    #[prost(int32, tag = "1")]
    pub my_int: i32,
    #[prost(bool, tag = "2")]
    pub my_bool: bool,
    #[prost(int32, optional, tag = "3")]
    pub my_nullable_int: Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub my_nullable_struct: Option<Foo>,
    #[prost(oneof = "Widget", tags = "5, 6")]
    pub my_enum: ::core::option::Option<Widget>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Message)]
pub struct Foo {
    #[prost(int32, tag = "1")]
    pub my_int: i32,
    #[prost(oneof = "Widget", tags = "5, 6")]
    pub widget: Option<Widget>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Oneof)]
pub enum Widget {
    #[prost(int32, tag = "5")]
    Quux(i32),
    #[prost(message, tag = "6")]
    Bar(String),
}

#[test]
fn derive_proto_test() {
    let my_struct = MyStruct {
        my_int: 1,
        my_bool: true,
        my_nullable_int: None,
        my_nullable_struct: Some(Foo {
            my_int: 1,
            widget: None,
        }),
        my_enum: Some(Widget::Quux(1)),
    };
    println!("{my_struct:?}");

    let buff = my_struct.encode_to_vec();
    println!("{buff:?}");
    assert_eq!(MyStruct::decode(&buff[..]).unwrap(), my_struct);

    let mut buff: Vec<u8> = Vec::new();
    my_struct.encode_length_delimited(&mut buff).unwrap();
    println!("{buff:?} (length delimited)");
    assert_eq!(
        MyStruct::decode_length_delimited(&buff[..]).unwrap(),
        my_struct
    );
}
