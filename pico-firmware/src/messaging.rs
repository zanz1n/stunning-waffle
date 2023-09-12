use serde::{ser::SerializeStruct, Serialize};

pub struct Payload {
    pub temperature_1: u16,
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Payload", 2)?;

        state.serialize_field("Temperature 1", &self.temperature_1)?;

        state.end()
    }
}
