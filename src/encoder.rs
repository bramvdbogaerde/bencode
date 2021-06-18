
use serde::{ser, ser::Impossible, Serialize};
use std::cell::RefCell;
use crate::errors::EncoderError;

struct Encoder {
    buffer: RefCell<Option<String>>,
}

impl Encoder {
    /// Extend the buffer with the given string
    fn extend_buffer<'s>(&self, s: &'s str) -> () {
        let mut o = self.buffer.borrow_mut();
        o.as_mut().unwrap().extend(s.chars())
    }

    fn new() -> Encoder {
        Encoder {
            buffer: RefCell::new(Some(String::new()))
        }
    }
}

/// Produces a bittorrent encoded dictionary
/// from a serializable structure
pub fn to_string<T: Serialize>(v: T) -> Result<String, EncoderError> {
    let encoder = Encoder::new();
    v.serialize(&encoder)?;
    let mut buffer = encoder.buffer.borrow_mut();
    Ok(buffer.take().unwrap())
}

impl<'a> serde::ser::Serializer for &'a Encoder {
    type Ok = ();
    type Error = EncoderError;
    type SerializeSeq = ListEncoder<'a>; // TODO
    type SerializeTuple = Impossible<Self::Ok, Self::Error>; // TODO
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>; // TODO
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>; // TODO
    type SerializeMap = DictEncoder<'a>;
    type SerializeStruct = DictEncoder<'a>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, EncoderError> {
        todo!()
    }

    fn serialize_bool(self, v: bool) -> Result<(), EncoderError> {
        // booleans are not officially supported by bencode, we just encode them
        // as integers 0 for false and 1 for true.
        self.extend_buffer(&format!("i{}e", if v { 1 } else { 0 }));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("i{}e", v));
        Ok(())
    }

    fn serialize_f32(self, _: f32) -> Result<(), EncoderError> {
        Err(EncoderError::FloatNotSupported)
    }

    fn serialize_f64(self, _: f64) -> Result<(), EncoderError> {
        Err(EncoderError::FloatNotSupported)
    }

    fn serialize_char(self, c: char) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("{}:{}", 1, c));
        Ok(())
    }

    fn serialize_str(self, s: &str) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("{}:{}", s.len(), s));
        Ok(())
    }

    fn serialize_bytes(self, _s: &[u8]) -> Result<(), EncoderError> {
        // TODO: this should be supported according to the specification
        // of the bencoder
        Err(EncoderError::NotSupported)
    }

    fn serialize_none(self) -> Result<(), EncoderError> {
        // TODO: check if this does not give any problems:
        // we don't need to serialize anything when serializing none
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, u: &T) -> Result<(), EncoderError> {
        u.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), EncoderError> {
        // nothing to serialize
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), EncoderError> {
        self.extend_buffer(&format!("{}:{}", name.len(), name));
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), EncoderError> {
        // TODO: we should check if this is deserializable, maybe we need to encode it is a
        // dictionary
        self.extend_buffer(&format!(
            "{}:{}{}",
            name.len() + variant.len(),
            name,
            variant
        ));
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), EncoderError> {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), EncoderError> {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, EncoderError> {
        Ok(ListEncoder::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, EncoderError> {
        self.extend_buffer("l");
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, EncoderError> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, EncoderError> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, EncoderError> {
        Ok(DictEncoder::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, EncoderError> {
        // TODO: do something with the name?
        Ok(DictEncoder::new(self))
    }
}

/// This struct is responsible for serializing keys in the dictionary,
/// it allows to serialize all kinds of primitive types as keys, but
/// not maps or structs
struct KeyEncoder<'a> {
    encoder: &'a Encoder,
}

impl<'a> KeyEncoder<'a> {
    fn new(encoder: &'a Encoder) -> KeyEncoder {
        KeyEncoder { encoder }
    }
}

impl<'a> ser::Serializer for &'a KeyEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", v));
        Ok(())
    }

    fn serialize_char(self, c: char) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", c));
        Ok(())
    }

    fn serialize_str(self, s: &str) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", s));
        Ok(())
    }

    fn serialize_bytes(self, _s: &[u8]) -> Result<(), EncoderError> {
        // TODO: this should be supported according to the specification
        // of the bencoder
        Err(EncoderError::NotSupported)
    }

    fn serialize_none(self) -> Result<(), EncoderError> {
        // TODO: check if this does not give any problems:
        // we don't need to serialize anything when serializing none
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, u: &T) -> Result<(), EncoderError> {
        u.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), EncoderError> {
        // nothing to serialize
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), EncoderError> {
        self.encoder.extend_buffer(&format!("{}", name));
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), EncoderError> {
        // TODO: we should check if this is deserializable, maybe we need to encode it is a
        // dictionary
        self.encoder.extend_buffer(&format!("{}{}", name, variant));
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), EncoderError> {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), EncoderError> {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, EncoderError> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, EncoderError> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, EncoderError> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, EncoderError> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, EncoderError> {
        Err(EncoderError::NotSupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, EncoderError> {
        Err(EncoderError::NotSupported)
    }
}

struct DictEncoder<'a> {
    encoder: &'a Encoder,
    key_encoder: KeyEncoder<'a>,
}

impl<'a> DictEncoder<'a> {
    fn new(encoder: &'a Encoder) -> DictEncoder<'a> {
        encoder.extend_buffer("d");
        DictEncoder {
            encoder,
            key_encoder: KeyEncoder::new(encoder),
        }
    }

    fn add_field<T>(&self, key: &'static str, value: &T) -> Result<(), EncoderError>
    where
        T: ?Sized + Serialize,
    {
        self.encoder.extend_buffer(key);
        value.serialize(self.encoder)
    }

    fn end_dict(&self) -> Result<(), EncoderError> {
        self.encoder.extend_buffer("e");
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for DictEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.add_field(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.end_dict()
    }

    fn skip_field(&mut self, _: &'static str) -> Result<(), Self::Error> {
        // TODO: not sure what this is
        todo!()
    }
}

impl<'a> ser::SerializeMap for DictEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // key can only be a string not an arbitrary datatype
        key.serialize(&self.key_encoder)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // value can be anything we will just serialize it
        value.serialize(self.encoder)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.end_dict()
    }
}

struct ListEncoder<'a> {
    encoder: &'a Encoder
}

impl<'a> ListEncoder<'a> {
    fn new(encoder: &'a Encoder) -> ListEncoder<'a> {
        encoder.extend_buffer("l");
        ListEncoder {
            encoder
        }
    }
}


impl<'a> ser::SerializeSeq for ListEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> 
        where T: ?Sized + Serialize {
        value.serialize(self.encoder)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.encoder.extend_buffer("e");
        Ok(())
    }
}

