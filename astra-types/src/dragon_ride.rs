use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct DragonRidePresetParamDataBook {
    pub dragon_ride_preset_param_data: Sheet<Vec<DragonRidePresetParamData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct DragonRidePresetParamData {
    #[astra(key = "@Group")]
    pub group: String,
    #[astra(key = "@IsTimeTest")]
    pub is_time_test: Option<i8>,
    #[astra(key = "@IsWalkThroughON")]
    pub is_walk_through_on: Option<i8>,
    #[astra(key = "@Course_1")]
    pub course_1: Option<i8>,
    #[astra(key = "@Stime_1")]
    pub stime_1: Option<f32>,
    #[astra(key = "@Srandom_1")]
    pub srandom_1: Option<f32>,
    #[astra(key = "@Course_2")]
    pub course_2: Option<i8>,
    #[astra(key = "@Stime_2")]
    pub stime_2: Option<f32>,
    #[astra(key = "@Srandom_2")]
    pub srandom_2: Option<f32>,
    #[astra(key = "@Course_3")]
    pub course_3: Option<i8>,
    #[astra(key = "@Stime_3")]
    pub stime_3: Option<f32>,
    #[astra(key = "@Srandom_3")]
    pub srandom_3: Option<f32>,
    #[astra(key = "@Course_4")]
    pub course_4: Option<i8>,
    #[astra(key = "@Stime_4")]
    pub stime_4: Option<f32>,
    #[astra(key = "@Srandom_4")]
    pub srandom_4: Option<f32>,
    #[astra(key = "@Course_5")]
    pub course_5: Option<i8>,
    #[astra(key = "@Stime_5")]
    pub stime_5: Option<f32>,
    #[astra(key = "@Srandom_5")]
    pub srandom_5: Option<f32>,
    #[astra(key = "@Course_6")]
    pub course_6: Option<i8>,
    #[astra(key = "@Stime_6")]
    pub stime_6: Option<f32>,
    #[astra(key = "@Srandom_6")]
    pub srandom_6: Option<f32>,
    #[astra(key = "@Course_7")]
    pub course_7: Option<i8>,
    #[astra(key = "@Stime_7")]
    pub stime_7: Option<f32>,
    #[astra(key = "@Srandom_7")]
    pub srandom_7: Option<f32>,
    #[astra(key = "@Course_8")]
    pub course_8: Option<i8>,
    #[astra(key = "@Stime_8")]
    pub stime_8: Option<f32>,
    #[astra(key = "@Srandom_8")]
    pub srandom_8: Option<f32>,
}

#[derive(AstraBook)]
pub struct DragonRidePrizeListBook {
    pub dragon_ride_prize_data: Sheet<Vec<DragonRidePrizeData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct DragonRidePrizeData {
    #[astra(key = "@Group")]
    pub group: String,
    #[astra(key = "@PieceOfBond")]
    pub piece_of_bond: Option<i32>,
    #[astra(key = "@ItemCount")]
    pub item_count: Option<i32>,
    #[astra(key = "@Item1")]
    pub item_1: String,
    #[astra(key = "@Item2")]
    pub item_2: String,
    #[astra(key = "@Item3")]
    pub item_3: String,
    #[astra(key = "@Item4")]
    pub item_4: String,
    #[astra(key = "@Item5")]
    pub item_5: String,
    #[astra(key = "@Item6")]
    pub item_6: String,
}

#[derive(AstraBook)]
pub struct DragonRideTargetPatternBook {
    pub dragon_ride_target_patterns: Sheet<Vec<DragonRideTargetPattern>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct DragonRideTargetPattern {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Target1")]
    pub target_1: Option<i8>,
    #[astra(key = "@Target2")]
    pub target_2: Option<i8>,
    #[astra(key = "@Target3")]
    pub target_3: Option<i8>,
    #[astra(key = "@Target4")]
    pub target_4: Option<i8>,
    #[astra(key = "@Target5")]
    pub target_5: Option<i8>,
    #[astra(key = "@Target6")]
    pub target_6: Option<i8>,
    #[astra(key = "@Target7")]
    pub target_7: Option<i8>,
    #[astra(key = "@Target8")]
    pub target_8: Option<i8>,
}
