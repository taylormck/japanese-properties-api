//! A data type to represent Japanese real estate properties

use serde::{ser::SerializeStruct, Serialize};

// TODO: using Strings is pretty safe, and avoids plenty of issues when
// we're only worried about converting between CSV and JSON data.
// However, it's likely using more memory than really necessary, so we
// should consider downsizing a bit, such as by using raw Bytes.

#[derive(Debug, Clone)]
pub struct Property {
    pub id: usize,
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub chome: String,
    pub banchi: String,
    pub go: String,
    pub building: String,
    pub price: String,
    pub nearest_station: String,
    pub property_type: String,
    pub land_area: String,
}

// We add a custom implementation of Serialize so that we
// can add the full_address property to the JSON
impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Property", 13)?;
        s.serialize_field("id", &self.id)?;

        // Here's our lovely custom field
        // This is the formal way to display Japanese addresses, though
        // there are a couple of other variations that could have been used.
        // For example, the chome, banchi, and go fields are sometimes displayed
        // as 1-2-3, or 1丁目1-2, etc.
        s.serialize_field(
            "full_address",
            &format!(
                "{}{}{}{}丁目{}番地{}号{}",
                &self.prefecture,
                &self.city,
                &self.town,
                &self.chome,
                &self.banchi,
                &self.go,
                &self.building,
            ),
        )?;

        s.serialize_field("prefecture", &self.prefecture)?;
        s.serialize_field("city", &self.city)?;
        s.serialize_field("town", &self.town)?;
        s.serialize_field("chome", &self.chome)?;
        s.serialize_field("banchi", &self.banchi)?;
        s.serialize_field("go", &self.go)?;
        s.serialize_field("building", &self.building)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("nearest_station", &self.nearest_station)?;
        s.serialize_field("property_type", &self.property_type)?;
        s.serialize_field("land_area", &self.land_area)?;

        s.end()
    }
}
