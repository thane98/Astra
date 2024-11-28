use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MuscleExerciseDataBook {
    pub difficulty: Sheet<IndexMap<String, MuscleExerciseDifficulty>>,
    pub setups: Sheet<IndexMap<String, MuscleExerciseSetup>>,
    pub prizes: Sheet<IndexMap<String, MuscleExercisePrizeData>>,
    pub sit_up_fall_data: Sheet<IndexMap<String, MuscleSitUpFallData>>,
    pub push_up_speed: Sheet<IndexMap<String, MusclePushUpSpeedData>>,
    pub squat_judge_area: Sheet<IndexMap<String, MuscleSquatJudgeAreaData>>,
    pub score_list_data: Sheet<IndexMap<String, MuscleSquatScoreListData>>,
    pub music_sheets: Sheet<IndexMap<String, Vec<MuscleSquatMusicSheet>>>,
    pub assist_data: Sheet<IndexMap<String, MuscleAssistData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExerciseDifficulty {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@ExerciseType")]
    pub exercise_type: i8,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@GoodScore")]
    pub good_score: i32,
    #[astra(key = "@PerfectScore")]
    pub perfect_score: i32,
    #[astra(key = "@TargetScore")]
    pub target_score: i32,
    #[astra(key = "@EndlessGoalCount")]
    pub endless_goal_count: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExerciseSetup {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@AreaP_Center")]
    pub area_p_center: f32,
    #[astra(key = "@AreaP_Radius")]
    pub area_p_radius: f32,
    #[astra(key = "@AreaG_Center")]
    pub area_g_center: f32,
    #[astra(key = "@AreaG_Radius")]
    pub area_g_radius: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExercisePrizeData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@ExerciseType")]
    pub exercise_type: i8,
    #[astra(key = "@Bonus_SSS")]
    pub bonus_sss: String,
    #[astra(key = "@Bond_SSS")]
    pub bond_sss: i32,
    #[astra(key = "@Border_SSS")]
    pub border_sss: i32,
    #[astra(key = "@Bonus_SS")]
    pub bonus_ss: String,
    #[astra(key = "@Bond_SS")]
    pub bond_ss: i32,
    #[astra(key = "@Border_SS")]
    pub border_ss: i32,
    #[astra(key = "@Bonus_S")]
    pub bonus_s: String,
    #[astra(key = "@Bond_S")]
    pub bond_s: i32,
    #[astra(key = "@Border_S")]
    pub border_s: i32,
    #[astra(key = "@Bonus_A")]
    pub bonus_a: String,
    #[astra(key = "@Bond_A")]
    pub bond_a: i32,
    #[astra(key = "@Border_A")]
    pub border_a: i32,
    #[astra(key = "@Bonus_B")]
    pub bonus_b: String,
    #[astra(key = "@Bond_B")]
    pub bond_b: i32,
    #[astra(key = "@Border_B")]
    pub border_b: i32,
    #[astra(key = "@Bonus_C")]
    pub bonus_c: String,
    #[astra(key = "@Bond_C")]
    pub bond_c: i32,
    #[astra(key = "@Border_C")]
    pub border_c: i32,
    #[astra(key = "@Bonus_D")]
    pub bonus_d: String,
    #[astra(key = "@Bond_D")]
    pub bond_d: i32,
    #[astra(key = "@Border_D")]
    pub border_d: i32,
    #[astra(key = "@Bonus_E")]
    pub bonus_e: String,
    #[astra(key = "@Bond_E")]
    pub bond_e: i32,
    #[astra(key = "@Border_E")]
    pub border_e: i32,
    #[astra(key = "@Bonus_F")]
    pub bonus_f: String,
    #[astra(key = "@Bond_F")]
    pub bond_f: i32,
    #[astra(key = "@Border_F")]
    pub border_f: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSitUpFallData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@PerfectLimit")]
    pub perfect_limit: f32,
    #[astra(key = "@GainPower")]
    pub gain_power: f32,
    #[astra(key = "@FallSpeed")]
    pub fall_speed: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MusclePushUpSpeedData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@SpeedMin")]
    pub speed_min: f32,
    #[astra(key = "@SpeedMax")]
    pub speed_max: f32,
    #[astra(key = "@LevelUpCount")]
    pub level_up_count: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatJudgeAreaData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@BadFrameFirst")]
    pub bad_frame_first: f32,
    #[astra(key = "@GoodFrameFirst")]
    pub good_frame_first: f32,
    #[astra(key = "@PerfectFrameFirst")]
    pub perfect_frame_first: f32,
    #[astra(key = "@PerfectFrameLatter")]
    pub perfect_frame_latter: f32,
    #[astra(key = "@GoodFrameLatter")]
    pub good_frame_latter: f32,
    #[astra(key = "@BadFrameLatter")]
    pub bad_frame_latter: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatScoreListData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@UseCount")]
    pub use_count: i32,
    #[astra(key = "@IsDoubleChoice")]
    pub is_double_choice: bool,
    #[astra(key = "@Speed")]
    pub speed: f32,
    #[astra(key = "@Length")]
    pub length: f32,
    #[astra(key = "@Type_1")]
    pub type_1: String,
    #[astra(key = "@LotteryParam_1")]
    pub lottery_param_1: f32,
    #[astra(key = "@Type_2")]
    pub type_2: String,
    #[astra(key = "@LotteryParam_2")]
    pub lottery_param_2: f32,
    #[astra(key = "@Type_3")]
    pub type_3: String,
    #[astra(key = "@LotteryParam_3")]
    pub lottery_param_3: f32,
    #[astra(key = "@Type_4")]
    pub type_4: String,
    #[astra(key = "@LotteryParam_4")]
    pub lottery_param_4: f32,
    #[astra(key = "@Type_5")]
    pub type_5: String,
    #[astra(key = "@LotteryParam_5")]
    pub lottery_param_5: f32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatMusicSheet {
    #[astra(key = "@ID", public_array)]
    pub id: String,
    #[astra(key = "@TypeA_L")]
    pub type_a_l: i8,
    #[astra(key = "@TypeA_R")]
    pub type_a_r: i8,
    #[astra(key = "@TypeB_L")]
    pub type_b_l: i8,
    #[astra(key = "@TypeB_R")]
    pub type_b_r: i8,
    #[astra(key = "@TypeC_L")]
    pub type_c_l: i8,
    #[astra(key = "@TypeC_R")]
    pub type_c_r: i8,
    #[astra(key = "@TypeD_L")]
    pub type_d_l: i8,
    #[astra(key = "@TypeD_R")]
    pub type_d_r: i8,
    #[astra(key = "@TypeE_L")]
    pub type_e_l: i8,
    #[astra(key = "@TypeE_R")]
    pub type_e_r: i8,
    #[astra(key = "@Ensure")]
    pub ensure: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleAssistData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Level_00")]
    pub level_00: i32,
    #[astra(key = "@Level_01")]
    pub level_01: i32,
    #[astra(key = "@Level_02")]
    pub level_02: i32,
    #[astra(key = "@Level_03")]
    pub level_03: i32,
    #[astra(key = "@Level_04")]
    pub level_04: i32,
    #[astra(key = "@Level_05")]
    pub level_05: i32,
    #[astra(key = "@Level_06")]
    pub level_06: i32,
    #[astra(key = "@Level_07")]
    pub level_07: i32,
    #[astra(key = "@Level_08")]
    pub level_08: i32,
    #[astra(key = "@Level_09")]
    pub level_09: i32,
    #[astra(key = "@Level_10")]
    pub level_10: i32,
}
