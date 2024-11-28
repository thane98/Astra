use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct FishingFishBook {
    pub fish: Sheet<IndexMap<String, FishingFishData>>,
    pub size_data: Sheet<IndexMap<String, FishSizeData>>,
    pub spawns: Sheet<IndexMap<String, FishSpawn>>,
    pub target_list: Sheet<IndexMap<String, FishingTargetListData>>,
    pub assist_data: Sheet<IndexMap<String, FishingAssistData>>,
    pub radical_param_data: Sheet<IndexMap<String, FishingRadicalParamData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishingFishData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@FishName")]
    pub fish_name: String,
    #[astra(key = "@LargeType")]
    pub large_type: i8,
    #[astra(key = "@ShadowSize")]
    pub shadow_size: i8,
    #[astra(key = "@RadarSizeMult")]
    pub radar_size_mult: f32,
    #[astra(key = "@FoodType")]
    pub food_type: String,
    #[astra(key = "@PieceCount")]
    pub piece_count: i32,
    #[astra(key = "@CounterTime")]
    pub counter_time: f32,
    #[astra(key = "@TurnCounterTime")]
    pub turn_counter_time: f32,
    #[astra(key = "@TurnAngleMin")]
    pub turn_angle_min: f32,
    #[astra(key = "@TurnAngleMax")]
    pub turn_angle_max: f32,
    #[astra(key = "@EscapeSpeed")]
    pub escape_speed: f32,
    #[astra(key = "@CounterSpeedH")]
    pub counter_speed_h: f32,
    #[astra(key = "@CounterSpeedM")]
    pub counter_speed_m: f32,
    #[astra(key = "@CounterSpeedL")]
    pub counter_speed_l: f32,
    #[astra(key = "@CatchTime")]
    pub catch_time: f32,
    #[astra(key = "@CatchTimeRandomAdd")]
    pub catch_time_random_add: f32,
    #[astra(key = "@EscapeTime")]
    pub escape_time: f32,
    #[astra(key = "@HP")]
    pub hp: f32,
    #[astra(key = "@LethalHP")]
    pub lethal_hp: f32,
    #[astra(key = "@RegenaratePerSec")]
    pub regenarate_per_sec: f32,
    #[astra(key = "@BaseSize")]
    pub base_size: f32,
    #[astra(key = "@NameLabel")]
    pub name_label: String,
    #[astra(key = "@MessageLabel")]
    pub message_label: String,
    #[astra(key = "@TimeFlagMorning")]
    pub time_flag_morning: bool,
    #[astra(key = "@TimeFlagDay")]
    pub time_flag_day: bool,
    #[astra(key = "@TimeFlagNight")]
    pub time_flag_night: bool,
    #[astra(key = "@BestRodType")]
    pub best_rod_type: i8,
    #[astra(key = "@TextureID")]
    pub texture_id: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishSizeData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@SizeName")]
    pub size_name: String,
    #[astra(key = "@SizeMinimum")]
    pub size_minimum: f32,
    #[astra(key = "@SizeMaximum")]
    pub size_maximum: f32,
    #[astra(key = "@BonusMinimum")]
    pub bonus_minimum: f32,
    #[astra(key = "@BonusMaximum")]
    pub bonus_maximum: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishSpawn {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@StickType")]
    pub stick_type: String,
    #[astra(key = "@Time")]
    pub time: i8,
    #[astra(key = "@PositionNum")]
    pub position_num: i32,
    #[astra(key = "@LotteryParam")]
    pub lottery_param: i32,
    #[astra(key = "@FishID")]
    pub fish_id: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishingTargetListData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@FishID")]
    pub fish_id: String,
    #[astra(key = "@Priority")]
    pub priority: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishingAssistData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level_00")]
    pub level_00: f32,
    #[astra(key = "@Level_01")]
    pub level_01: f32,
    #[astra(key = "@Level_02")]
    pub level_02: f32,
    #[astra(key = "@Level_03")]
    pub level_03: f32,
    #[astra(key = "@Level_04")]
    pub level_04: f32,
    #[astra(key = "@Level_05")]
    pub level_05: f32,
    #[astra(key = "@Level_06")]
    pub level_06: f32,
    #[astra(key = "@Level_07")]
    pub level_07: f32,
    #[astra(key = "@Level_08")]
    pub level_08: f32,
    #[astra(key = "@Level_09")]
    pub level_09: f32,
    #[astra(key = "@Level_10")]
    pub level_10: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FishingRadicalParamData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Sec_01")]
    pub sec_01: f32,
    #[astra(key = "@Power_01")]
    pub power_01: f32,
    #[astra(key = "@Regene_01")]
    pub regene_01: f32,
    #[astra(key = "@Sec_02")]
    pub sec_02: f32,
    #[astra(key = "@Power_02")]
    pub power_02: f32,
    #[astra(key = "@Regene_02")]
    pub regene_02: f32,
    #[astra(key = "@Sec_03")]
    pub sec_03: f32,
    #[astra(key = "@Power_03")]
    pub power_03: f32,
    #[astra(key = "@Regene_03")]
    pub regene_03: f32,
    #[astra(key = "@Sec_04")]
    pub sec_04: f32,
    #[astra(key = "@Power_04")]
    pub power_04: f32,
    #[astra(key = "@Regene_04")]
    pub regene_04: f32,
}
