use serde::{de, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Payload(pub HashMap<String, f32>);

impl Default for Payload {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PayloadVisitor;

        impl<'de> de::Visitor<'de> for PayloadVisitor {
            type Value = Payload;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string map with u16 values")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hashmap = if let Some(size) = map.size_hint() {
                    HashMap::with_capacity(size)
                } else {
                    HashMap::new()
                };

                while let Some((key, value)) = map.next_entry::<String, f32>()? {
                    hashmap.insert(key, value);
                }

                Ok(Payload(hashmap))
            }
        }

        deserializer.deserialize_map(PayloadVisitor)
    }
}

impl Serialize for Payload {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_map(Some(self.0.len()))?;

        for (key, value) in self.0.iter() {
            state.serialize_entry(key, value)?;
        }

        state.end()
    }
}
