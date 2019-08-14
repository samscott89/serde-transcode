//! Transcode from one Serde format to another.
//!
//! This crate provides functionality to "transcode" from an arbitrary Serde
//! `Deserializer` to an arbitrary Serde `Serializer` without needing to
//! collect the entire input into an intermediate form in memory. For example,
//! you could translate a stream of JSON data into a stream of CBOR data, or
//! translate JSON into its pretty-printed form.
//!
//! # Examples
//!
//! Translate a JSON file to a pretty-printed version.
//!
//! ```no_run
//! extern crate serde;
//! extern crate serde_json;
//! extern crate serde_transcode;
//!
//! use serde::Serialize;
//! use serde_json::{Serializer, Deserializer};
//! use std::io::{Read, Write, BufReader, BufWriter};
//! use std::fs::File;
//!
//! fn main() {
//!     let reader = BufReader::new(File::open("input.json").unwrap());
//!     let writer = BufWriter::new(File::create("output.json").unwrap());
//!
//!     let mut deserializer = Deserializer::from_reader(reader);
//!     let mut serializer = Serializer::pretty(writer);
//!     serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
//!     serializer.into_inner().flush().unwrap();
//! }
//! ```
#![warn(missing_docs)]
#![doc(html_root_url="https://docs.rs/serde-transcode/1.0.1")]

#[macro_use]
extern crate serde;

use serde::de::IntoDeserializer;
use serde::de;
use serde::ser::{self, Serialize, SerializeSeq, SerializeMap};
use std::cell::RefCell;
use std::fmt;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct RedocError;

impl std::fmt::Display for RedocError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "RedocError")
    }
}

impl std::error::Error for RedocError {

}

impl serde::de::Error for RedocError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display
    {
        Self
    }
}

impl serde::ser::Error for RedocError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display
    {
        Self
    }
}

pub fn edocsnart<'de, D, S>(s: S) -> Result<D, RedocError>
where
    D: de::Deserialize<'de>,
    S: ser::Serialize,
{
    let mut redoc = Redocsnart::new();
    // s.serialize(redoc);
    // redoc.deserialize()
    unimplemented!()

}

/// Transcodes from a Serde `Deserializer` to a Serde `Serializer`.
pub fn transcode<'de, D, S>(d: D, s: S) -> Result<S::Ok, S::Error>
    where D: de::Deserializer<'de>,
          S: ser::Serializer
{
    Transcoder::new(d).serialize(s)
}

use serde::de::value::*;

pub struct Redocsnart<'de>(Box<erased_serde::Deserializer<'de>>);

impl Default for Redocsnart<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Redocsnart<'_> {
    pub fn new() -> Self{ 
        // Self(Box::new(erased_serde::Deserializer::erase::<UnitDeserializer<RedocError>>(().into_deserializer())))
        Self::from(())
    }
}

impl<'de, D> From<D> for Redocsnart<'de>
where
    D: IntoDeserializer<'de, RedocError>
{
    fn from(other: D) -> Self {
        Self(Box::new(erased_serde::Deserializer::erase(other.into_deserializer())))
    }
}

impl<'de> ser::Serializer for Redocsnart<'de>
{
    type Ok = ();
    type Error = RedocError;
    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = &'de mut Self;
    type SerializeTuple = &'de mut Self;

    type SerializeTupleStruct = &'de mut Self;

    type SerializeTupleVariant = &'de mut Self;

    type SerializeMap = &'de mut Self;

    type SerializeStruct = &'de mut Self;

    type SerializeStructVariant = &'de mut Self;


    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), RedocError> {
        self = Self::from(v);
                Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), RedocError> {
        self = Self::from(v);
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<(), RedocError> {
        self = Self::from(v);
        Ok(())
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<(), RedocError> {
        self = Self::from(None);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        self = Self::from(value);
        Ok(())
    }

    fn serialize_unit(self) -> Result<(), RedocError> {
        self = Self::from(());
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), RedocError> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), RedocError> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":";
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, RedocError> {
        self.output += "[";
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, RedocError> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, RedocError> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, RedocError> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, RedocError> {
        self.output += "{";
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, RedocError> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, RedocError> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}



// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Redocsnart<'a> {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = RedocError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<(), RedocError> {
        self.output += "]";
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "]";
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "]";
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "}";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Redocsnart<'a> {
    type Ok = ();
    type Error = RedocError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), RedocError>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), RedocError> {
        self.output += "}}";
        Ok(())
    }
}


// impl de::Deserializer<'de> for Redocsnart<S>
// where
//     S: ser::Serializer
// {
//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
//         V: de::Visitor<'de>
//     {
//         self.serializer.serialize(visitor)
//     }
// }

/// A Serde transcoder.
///
/// In most cases, the `transcode` function should be used instead of this
/// type.
///
/// # Note
///
/// Unlike traditional serializable types, `Transcoder`'s `Serialize`
/// implementation is *not* idempotent, as it advances the state of its
/// internal `Deserializer`. It should only ever be serialized once.
pub struct Transcoder<D>(RefCell<Option<D>>);

impl<'de, D> Transcoder<D>
    where D: de::Deserializer<'de>
{
    /// Constructs a new `Transcoder`.
    pub fn new(d: D) -> Transcoder<D> {
        Transcoder(RefCell::new(Some(d)))
    }
}

impl<'de, D> ser::Serialize for Transcoder<D>
    where D: de::Deserializer<'de>
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        self.0.borrow_mut().take().unwrap().deserialize_any(Visitor(s)).map_err(d2s)
    }
}

struct Visitor<S>(S);

impl<'de, S> de::Visitor<'de> for Visitor<S>
    where S: ser::Serializer
{
    type Value = S::Ok;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "any value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_bool(v).map_err(s2d)
    }

    fn visit_i8<E>(self, v: i8) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_i8(v).map_err(s2d)
    }

    fn visit_i16<E>(self, v: i16) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_i16(v).map_err(s2d)
    }

    fn visit_i32<E>(self, v: i32) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_i32(v).map_err(s2d)
    }

    fn visit_i64<E>(self, v: i64) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_i64(v).map_err(s2d)
    }

    fn visit_u8<E>(self, v: u8) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_u8(v).map_err(s2d)
    }

    fn visit_u16<E>(self, v: u16) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_u16(v).map_err(s2d)
    }

    fn visit_u32<E>(self, v: u32) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_u32(v).map_err(s2d)
    }

    fn visit_u64<E>(self, v: u64) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_u64(v).map_err(s2d)
    }

    serde_if_integer128! {
        fn visit_i128<E>(self, v: i128) -> Result<S::Ok, E>
            where E: de::Error
        {
            self.0.serialize_i128(v).map_err(s2d)
        }

        fn visit_u128<E>(self, v: u128) -> Result<S::Ok, E>
            where E: de::Error
        {
            self.0.serialize_u128(v).map_err(s2d)
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_f32(v).map_err(s2d)
    }

    fn visit_f64<E>(self, v: f64) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_f64(v).map_err(s2d)
    }

    fn visit_char<E>(self, v: char) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_char(v).map_err(s2d)
    }

    fn visit_str<E>(self, v: &str) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_str(v).map_err(s2d)
    }

    fn visit_string<E>(self, v: String) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_str(&v).map_err(s2d)
    }

    fn visit_unit<E>(self) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_unit().map_err(s2d)
    }

    fn visit_none<E>(self) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_none().map_err(s2d)
    }

    fn visit_some<D>(self, d: D) -> Result<S::Ok, D::Error>
        where D: de::Deserializer<'de>
    {
        self.0.serialize_some(&Transcoder::new(d)).map_err(s2d)
    }

    fn visit_newtype_struct<D>(self, d: D) -> Result<S::Ok, D::Error>
        where D: de::Deserializer<'de>
    {
        self.0.serialize_newtype_struct("<unknown>", &Transcoder::new(d)).map_err(s2d)
    }

    fn visit_seq<V>(self, mut v: V) -> Result<S::Ok, V::Error>
        where V: de::SeqAccess<'de>
    {
        let mut s = self.0.serialize_seq(v.size_hint()).map_err(s2d)?;
        while let Some(()) = v.next_element_seed(SeqSeed(&mut s))? {}
        s.end().map_err(s2d)
    }

    fn visit_map<V>(self, mut v: V) -> Result<S::Ok, V::Error>
        where V: de::MapAccess<'de>
    {
        let mut s = self.0.serialize_map(v.size_hint()).map_err(s2d)?;
        while let Some(()) = v.next_key_seed(KeySeed(&mut s))? {
            v.next_value_seed(ValueSeed(&mut s))?;
        }
        s.end().map_err(s2d)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_bytes(v).map_err(s2d)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<S::Ok, E>
        where E: de::Error
    {
        self.0.serialize_bytes(&v).map_err(s2d)
    }
}

struct SeqSeed<'a, S: 'a>(&'a mut S);

impl<'de, 'a, S> de::DeserializeSeed<'de> for SeqSeed<'a, S>
    where S: ser::SerializeSeq
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
        where D: de::Deserializer<'de>
    {
        self.0.serialize_element(&Transcoder::new(deserializer)).map_err(s2d)
    }
}

struct KeySeed<'a, S: 'a>(&'a mut S);

impl<'de, 'a, S> de::DeserializeSeed<'de> for KeySeed<'a, S>
    where S: ser::SerializeMap
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
        where D: de::Deserializer<'de>
    {
        self.0.serialize_key(&Transcoder::new(deserializer)).map_err(s2d)
    }
}

struct ValueSeed<'a, S: 'a>(&'a mut S);

impl<'de, 'a, S> de::DeserializeSeed<'de> for ValueSeed<'a, S>
    where S: ser::SerializeMap
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
        where D: de::Deserializer<'de>
    {
        self.0.serialize_value(&Transcoder::new(deserializer)).map_err(s2d)
    }
}

fn d2s<D, S>(d: D) -> S
    where D: de::Error,
          S: ser::Error
{
    S::custom(d.to_string())
}

fn s2d<S, D>(s: S) -> D
    where S: ser::Error,
          D: de::Error
{
    D::custom(s.to_string())
}
