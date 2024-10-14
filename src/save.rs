use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// Define the Position struct
#[derive(Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

// Implement the Position encoding and decoding
impl Position {
    // Encodes the Position into a 64-bit long value
    fn encode(&self) -> i64 {
        let x_encoded = ((self.x & 0x3FFFFFF) as i64) << 38;
        let y_encoded = (self.y & 0xFFF) as i64;
        let z_encoded = ((self.z & 0x3FFFFFF) as i64) << 12;
        x_encoded | z_encoded | y_encoded
    }

    // Decodes a 64-bit long value into a Position
    fn decode(val: i64) -> Self {
        let x = (val >> 38) & 0x3FFFFFF;
        let y = val & 0xFFF;
        let z = (val >> 12) & 0x3FFFFFF;
        let x = if x >= (1 << 25) { x - (1 << 26) } else { x }; // sign extend for x
        let y = if y >= (1 << 11) { y - (1 << 12) } else { y }; // sign extend for y
        let z = if z >= (1 << 25) { z - (1 << 26) } else { z }; // sign extend for z

        Position { x, y, z }
    }
}

// Implement the Serialize trait for Position
impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = self.encode();
        serializer.serialize_i64(encoded)
    }
}

// Implement the Deserialize trait for Position, using a raw i64
impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(PositionVisitor)
    }
}

// Define a visitor for the Position struct to handle i64 deserialization
struct PositionVisitor;

impl<'de> Visitor<'de> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64-bit encoded position as an integer")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Position::decode(value))
    }

    // In case we get an unsigned value (u64), handle the conversion
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value > i64::MAX as u64 {
            Err(E::custom("value exceeds 64-bit signed integer range"))
        } else {
            Ok(Position::decode(value as i64))
        }
    }
}

fn main() {
    // Example using i64 for x, y, and z
    let position = Position { x: 18357644, y: 831, z: -20882616 };
    let encoded_position = position.encode();
    println!("Encoded Position as number: {}", encoded_position);

    // Deserialize from the raw i64 number directly
    let encoded: i64 = 5046110948485792575;
    let deserialized: Position = serde_json::from_str(&encoded.to_string()).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Ensure that serialization and deserialization are consistent
    assert_eq!(position, deserialized);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let position = Position { x: 18357644, y: 831, z: -20882616 };
        let encoded = position.encode();
        assert_eq!(encoded, 5046110948485792575);
    }

    #[test]
    fn test_deserialization() {
        let encoded: i64 = 5046110948485792575;
        let deserialized: Position = serde_json::from_str(&encoded.to_string()).unwrap();
        assert_eq!(deserialized, Position { x: 18357644, y: 831, z: -20882616 });
    }

    #[test]
    fn test_encoding_decoding() {
        let position = Position { x: 18357644, y: 831, z: -20882616 };
        let encoded = position.encode();
        let decoded = Position::decode(encoded);
        assert_eq!(position, decoded);
    }
}
