//! A data type to represent Japanese real estate properties

use serde::{ser::SerializeStruct, Serialize};

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
        s.serialize_field(
            "full_address",
            &format!(
                "{}{}{}{}{}{}{}",
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
