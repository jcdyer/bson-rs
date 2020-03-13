use crate::{
    Bson,
    ordered::OrderedDocument,
    spec::BinarySubtype,
};

use proptest::prelude::*;


fn arb_binary_subtype() -> impl Strategy<Value = BinarySubtype> {
    prop_oneof![
        Just(BinarySubtype::Generic),
        Just(BinarySubtype::Function),
        Just(BinarySubtype::BinaryOld),
        Just(BinarySubtype::UuidOld),
        Just(BinarySubtype::Uuid),
        Just(BinarySubtype::Md5),
    ]
}

pub(crate) fn arb_bson() -> impl Strategy<Value = Bson> {
    let leaf = prop_oneof![
        Just(Bson::Null),
        any::<String>().prop_map(Bson::String),
        any::<bool>().prop_map(Bson::Boolean),
        any::<f64>().prop_map(Bson::FloatingPoint),
        any::<i32>().prop_map(Bson::I32),
        any::<i64>().prop_map(Bson::I64),
        (arb_binary_subtype(), any::<Vec<u8>>()).prop_map(|(subtype, v)| Bson::Binary(subtype, v)),
    ];

    leaf.prop_recursive(
        4,
        256,
        10,
        |inner| prop_oneof![
            //prop::collection::vec(inner.clone(), 0..10).prop_map(Bson::Array),
            prop::collection::hash_map("[^\0]*", inner.clone(), 0..10).prop_map(|map| Bson::Document(map.into_iter().collect())),
        ]
    )
}
