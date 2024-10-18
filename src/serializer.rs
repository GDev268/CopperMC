use crate::buffer::{BufferError, ProtocolBuffer};
use serde::{
    ser::{self, Error},
    Serialize,
};
pub struct Serializer {
    pub output: ProtocolBuffer,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = BufferError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = BufferError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// Finish serializing a sequence.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a tuple element.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// Finish serializing a tuple.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//Need to make this for simpler structs
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a tuple struct field.
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Finish serializing a tuple struct.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a tuple variant field.
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Finish serializing a tuple variant.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a map key.
    ///
    /// If possible, `Serialize` implementations are encouraged to use
    /// `serialize_entry` instead as it may be implemented more efficiently in
    /// some formats compared to a pair of calls to `serialize_key` and
    /// `serialize_value`.
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Serialize a map value.
    ///
    /// # Panics
    ///
    /// Calling `serialize_value` before `serialize_key` is incorrect and is
    /// allowed to panic or produce bogus results.
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Serialize a map entry consisting of a key and a value.
    ///
    /// Some [`Serialize`] types are not able to hold a key and value in memory
    /// at the same time so `SerializeMap` implementations are required to
    /// support [`serialize_key`] and [`serialize_value`] individually. The
    /// `serialize_entry` method allows serializers to optimize for the case
    /// where key and value are both available. [`Serialize`] implementations
    /// are encouraged to use `serialize_entry` if possible.
    ///
    /// The default implementation delegates to [`serialize_key`] and
    /// [`serialize_value`]. This is appropriate for serializers that do not
    /// care about performance or are not able to optimize `serialize_entry` any
    /// better than this.
    ///
    /// [`Serialize`]: ../trait.Serialize.html
    /// [`serialize_key`]: #tymethod.serialize_key
    /// [`serialize_value`]: #tymethod.serialize_value
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Finish serializing a map.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a struct field.
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// Indicate that a struct field has been skipped.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }

    /// Finish serializing a struct.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok = ();

    /// Must match the `Error` type of our `Serializer`.
    type Error = BufferError;

    /// Serialize a struct variant field.
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    /// Indicate that a struct variant field has been skipped.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }

    /// Finish serializing a struct variant.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
