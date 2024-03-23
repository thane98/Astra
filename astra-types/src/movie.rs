use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MovieBook {
    pub movies: Sheet<IndexMap<String, Movie>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct Movie {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@MovieFileName", id)]
    pub movie_file_name: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Condition")]
    pub condition: String,
    #[astra(key = "@No")]
    pub no: Option<i8>,
    #[astra(key = "@BeforeSoundEventName1")]
    pub before_sound_event_name_1: String,
    #[astra(key = "@BeforeSoundEventName2")]
    pub before_sound_event_name_2: String,
    #[astra(key = "@BeforeSoundEventName3")]
    pub before_sound_event_name_3: String,
    #[astra(key = "@AfterSoundEventName1")]
    pub after_sound_event_name_1: String,
    #[astra(key = "@AfterSoundEventName2")]
    pub after_sound_event_name_2: String,
    #[astra(key = "@AfterSoundEventName3")]
    pub after_sound_event_name_3: String,
    #[astra(key = "@MessFileName")]
    pub mess_file_name: String,
    #[astra(key = "@DLCDirectoryName")]
    pub dlc_directory_name: String,
}
