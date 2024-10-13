use serde::{de::{self, MapAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use std::fmt;

// Define the Person struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

// Implement the Serialize trait for Person
impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("email", &self.email)?;
        state.end()
    }
}

// Implement the Deserialize trait for Person
impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PersonVisitor)
    }
}

// Define a visitor for the Person struct
struct PersonVisitor;

impl<'de> Visitor<'de> for PersonVisitor {
    type Value = Person;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with name, age, and email fields")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut name = None;
        let mut age = None;
        let mut email = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                "age" => {
                    if age.is_some() {
                        return Err(de::Error::duplicate_field("age"));
                    }
                    age = Some(map.next_value()?);
                }
                "email" => {
                    if email.is_some() {
                        return Err(de::Error::duplicate_field("email"));
                    }
                    email = Some(map.next_value()?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?; // Ignore unknown fields
                }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
        let email = email.ok_or_else(|| de::Error::missing_field("email"))?;

        Ok(Person { name, age, email })
    }
}

fn main() {
    let person = Person {
        name: String::from("Murat"),
        age: 20,
        email: String::from("murat@example.com"),
    };

    // Serialize the struct to a JSON string
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back to a struct
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        };
        let serialized = serde_json::to_string(&person).unwrap();
        assert!(serialized.contains("Alice"));
        assert!(serialized.contains("30"));
        assert!(serialized.contains("alice@example.com"));
    }

    #[test]
    fn test_deserialization() {
        let json_data = r#"{"name":"Bob","age":25,"email":"bob@example.com"}"#;
        let deserialized: Person = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized.name, "Bob");
        assert_eq!(deserialized.age, 25);
        assert_eq!(deserialized.email, "bob@example.com");
    }
}