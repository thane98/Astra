use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct VibrationBook {
    pub vibration_data: Sheet<Vec<VibrationDefineData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct VibrationDefineData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@EventName")]
    pub event_name: String,
    #[astra(key = "@VibrationFileName")]
    pub vibration_file_name: String,
    #[astra(key = "@AmplitudeMagnitude")]
    pub amplitude_magnitude: Option<f32>,
}