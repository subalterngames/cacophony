use serde::de::{Error, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// A value that is expressed as a u64 or an f32.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct U64orF32 {
    /// The value as a u64.
    u: u64,
    /// The value as an f32.
    f: f32,
}

impl U64orF32 {
    /// Returns the value as a u64.
    pub fn get_u(&self) -> u64 {
        self.u
    }

    /// Returns the value as an f32.
    pub fn get_f(&self) -> f32 {
        self.f
    }

    pub fn set(&mut self, value: u64) {
        self.u = value;
        self.f = value as f32;
    }
}

impl Eq for U64orF32 {}

impl From<u64> for U64orF32 {
    fn from(value: u64) -> Self {
        Self {
            u: value,
            f: value as f32,
        }
    }
}

impl From<f32> for U64orF32 {
    fn from(value: f32) -> Self {
        let u = value as u64;
        Self { u, f: u as f32 }
    }
}

impl Display for U64orF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.u)
    }
}

impl Serialize for U64orF32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.u)
    }
}

impl<'de> Visitor<'de> for U64orF32 {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u64")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::from(u64::from(value)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::from(u64::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::from(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::from(value))
    }
}

impl<'de> Deserialize<'de> for U64orF32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(U64orF32::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::U64orF32;
    use serde_json::{from_str, to_string, Error};

    #[test]
    fn set_uf() {
        uf_eq(U64orF32::from(1));
        uf_eq(U64orF32::from(1.0));
        uf_eq(U64orF32::from(-1.0));
        uf_eq(U64orF32::from(2.5));
    }

    #[test]
    fn uf_serialization() {
        let r = to_string(&U64orF32::from(5));
        assert!(r.is_ok());
        let s = r.unwrap();
        assert_eq!(&s, "5", "{}", s);
    }

    #[test]
    fn uf_deserialization() {
        deserialize_uf("5", U64orF32::from(5));
        deserialize_uf("5", U64orF32::from(5.0));
    }

    fn uf_eq(v: U64orF32) {
        assert_eq!(v.u as f32, v.f, "{:?}", v);
    }

    fn deserialize_uf(s: &str, v: U64orF32) {
        let r: Result<U64orF32, Error> = from_str(s);
        assert!(r.is_ok(), "{}", s);
        let q = r.unwrap();
        assert_eq!(q, v, "{:?} {:?}", q, v);
    }
}
