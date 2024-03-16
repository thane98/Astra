use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;


#[derive(AstraBook)]
pub struct EffectBook {
    pub effects: Sheet<IndexMap<String, Effect>>,
    pub effect_sequences: Sheet<IndexMap<String, EffectSequence>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct Effect {
    #[astra(key = "@Eid", id)]
    pub eid: String,
    #[astra(key = "@FilePath")]
    pub file_path: String,
    #[astra(key = "@SoundLabel")]
    pub sound_label: String,
    #[astra(key = "@Type")]
    pub ty: Option<i8>,
    #[astra(key = "@Resident")]
    pub resident: Option<i8>,
    #[astra(key = "@DelayTime")]
    pub delay_time: Option<f32>,
    #[astra(key = "@WaitTime")]
    pub wait_time: Option<f32>,
    #[astra(key = "@ShakeTime")]
    pub shake_time: Option<f32>,
    #[astra(key = "@ShakeMagnitude")]
    pub shake_magnitude: Option<f32>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EffectSequence {
    #[astra(key = "@Sequence", id)]
    pub sequence: String,
    #[astra(key = "@Active")]
    pub active: String,
    #[astra(key = "@Shoot")]
    pub shoot: String,
    #[astra(key = "@Hit")]
    pub hit: String,
}
