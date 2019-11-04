use serde::de::{DeserializeSeed, Deserializer, MapAccess, Visitor};

use super::Error;
use crate::raw::{ RawBsonBinary};
use crate::spec::BinarySubtype;

pub static SUBTYPE_FIELD: &str = "$__bson_binary_subtype";
pub static DATA_FIELD: &str = "$__bson_binary_data";
pub static NAME: &str = "$__bson_Binary";

pub(super) struct BinaryDeserializer<'de> {
    binary: RawBsonBinary<'de>,
    visited: u8,
}

impl<'de> BinaryDeserializer<'de> {
    pub(super) fn new(binary: RawBsonBinary<'de>) -> BinaryDeserializer<'de> {
        BinaryDeserializer { binary, visited: 0 }
    }
}

impl<'de> Deserializer<'de> for BinaryDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.binary.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_bytes(self.binary.as_bytes())
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_map(self)
    }

    serde::forward_to_deserialize_any!(
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    );
}

impl<'de> MapAccess<'de> for BinaryDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.visited {
            0 => seed.deserialize(BinaryKeyDeserializer::new(SUBTYPE_FIELD)).map(Some),
            1 => seed.deserialize(BinaryKeyDeserializer::new(DATA_FIELD)).map(Some),
            _ => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.visited {
            0 => {
                self.visited += 1;
                seed.deserialize(BinarySubtypeDeserializer::new(self.binary.subtype()))
            }
            1 => {
                self.visited += 1;
                seed.deserialize(BinaryDataDeserializer::new(self.binary))
            }
            _ => Err(Error::MalformedDocument),
        }
    }
}

struct BinaryKeyDeserializer {
    key: &'static str,
}

impl BinaryKeyDeserializer {
    fn new(key: &'static str) -> BinaryKeyDeserializer {
        BinaryKeyDeserializer { key }
    }
}

impl<'de> Deserializer<'de> for BinaryKeyDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.key)
    }

    serde::forward_to_deserialize_any!(
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    );
}

struct BinarySubtypeDeserializer {
    subtype: BinarySubtype,
}

impl BinarySubtypeDeserializer {
    fn new(subtype: BinarySubtype) -> BinarySubtypeDeserializer {
        BinarySubtypeDeserializer { subtype }
    }
}

impl<'de> Deserializer<'de> for BinarySubtypeDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.subtype.into())
    }

    serde::forward_to_deserialize_any!(
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    );
}

struct BinaryDataDeserializer<'de> {
    binary: RawBsonBinary<'de>,
}

impl<'de> BinaryDataDeserializer<'de> {
    fn new(binary: RawBsonBinary<'de>) -> BinaryDataDeserializer<'de> {
        BinaryDataDeserializer { binary }
    }
}

impl<'de> Deserializer<'de> for BinaryDataDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.binary.as_bytes())
    }

    serde::forward_to_deserialize_any!(
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    );
}