use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct GroundAttributeBook {
    pub ground_attributes: Sheet<Vec<GroundAttribute>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GroundAttribute {
    #[astra(key = "@Label")]
    pub label: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Sound")]
    pub sound: String,
    #[astra(key = "@Particle")]
    pub particle: String,
}