use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct SoundEventBook {
    pub sound_events: Sheet<Vec<SoundEvent>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct SoundEvent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@MovieFileName")]
    pub movie_file_name: String,
    #[astra(key = "@EventName1")]
    pub event_name_1: String,
    #[astra(key = "@EventName2")]
    pub event_name_2: String,
    #[astra(key = "@EventName3")]
    pub event_name_3: String,
    #[astra(key = "@EventName4")]
    pub event_name_4: String,
}