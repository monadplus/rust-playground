pub mod arc_swap;
pub mod box_dropping;
pub mod box_future;
pub mod const_generic_parameters;
pub mod cow;
pub mod decimal;
pub mod derive_proto;
pub mod dyn_trait;
pub mod experiment_1;
pub mod experiment_2;
pub mod experiment_3;
pub mod experiment_4;
pub mod iter_tests;
pub mod json_schema;
pub mod maybe_uninit;
pub mod must_use;
pub mod opentelemetry;
pub mod sequence_traverse;
pub mod stream;
pub mod thiserror;
pub mod tokio_stream_test;
pub mod tower;
pub mod tracing;
pub mod unordered_futures;
pub mod unsafe_rust;
pub mod template;

pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/examples.rs"));
    }
}

use prost::Message;
use snazzy::items;
use std::io::Cursor;

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(&mut Cursor::new(buf))
}
