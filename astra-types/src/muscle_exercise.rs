use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MuscleExerciseDataBook {
    pub difficulty: Sheet<Vec<MuscleExerciseDifficulty>>,
    pub setups: Sheet<Vec<MuscleExerciseSetup>>,
    pub prizes: Sheet<Vec<MuscleExercisePrizeData>>,
    pub sit_up_fall_data: Sheet<Vec<MuscleSitUpFallData>>,
    pub push_up_speed: Sheet<Vec<MusclePushUpSpeedData>>,
    pub squat_judge_area: Sheet<Vec<MuscleSquatJudgeAreaData>>,
    pub score_list_data: Sheet<Vec<MuscleSquatScoreListData>>,
    pub music_sheets: Sheet<Vec<MuscleSquatMusicSheet>>,
    pub assist_data: Sheet<Vec<MuscleAssistData>>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExerciseDifficulty {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@ExerciseType")]
    pub exercise_type: Option<i8>,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@GoodScore")]
    pub good_score: Option<i32>,
    #[astra(key = "@PerfectScore")]
    pub perfect_score: Option<i32>,
    #[astra(key = "@TargetScore")]
    pub target_score: Option<i32>,
    #[astra(key = "@EndlessGoalCount")]
    pub endless_goal_count: Option<i32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExerciseSetup {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@AreaP_Center")]
    pub area_p_center: Option<f32>,
    #[astra(key = "@AreaP_Radius")]
    pub area_p_radius: Option<f32>,
    #[astra(key = "@AreaG_Center")]
    pub area_g_center: Option<f32>,
    #[astra(key = "@AreaG_Radius")]
    pub area_g_radius: Option<f32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleExercisePrizeData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@ExerciseType")]
    pub exercise_type: Option<i8>,
    #[astra(key = "@Bonus_SSS")]
    pub bonus_sss: String,
    #[astra(key = "@Bond_SSS")]
    pub bond_sss: Option<i32>,
    #[astra(key = "@Border_SSS")]
    pub border_sss: Option<i32>,
    #[astra(key = "@Bonus_SS")]
    pub bonus_ss: String,
    #[astra(key = "@Bond_SS")]
    pub bond_ss: Option<i32>,
    #[astra(key = "@Border_SS")]
    pub border_ss: Option<i32>,
    #[astra(key = "@Bonus_S")]
    pub bonus_s: String,
    #[astra(key = "@Bond_S")]
    pub bond_s: Option<i32>,
    #[astra(key = "@Border_S")]
    pub border_s: Option<i32>,
    #[astra(key = "@Bonus_A")]
    pub bonus_a: String,
    #[astra(key = "@Bond_A")]
    pub bond_a: Option<i32>,
    #[astra(key = "@Border_A")]
    pub border_a: Option<i32>,
    #[astra(key = "@Bonus_B")]
    pub bonus_b: String,
    #[astra(key = "@Bond_B")]
    pub bond_b: Option<i32>,
    #[astra(key = "@Border_B")]
    pub border_b: Option<i32>,
    #[astra(key = "@Bonus_C")]
    pub bonus_c: String,
    #[astra(key = "@Bond_C")]
    pub bond_c: Option<i32>,
    #[astra(key = "@Border_C")]
    pub border_c: Option<i32>,
    #[astra(key = "@Bonus_D")]
    pub bonus_d: String,
    #[astra(key = "@Bond_D")]
    pub bond_d: Option<i32>,
    #[astra(key = "@Border_D")]
    pub border_d: Option<i32>,
    #[astra(key = "@Bonus_E")]
    pub bonus_e: String,
    #[astra(key = "@Bond_E")]
    pub bond_e: Option<i32>,
    #[astra(key = "@Border_E")]
    pub border_e: Option<i32>,
    #[astra(key = "@Bonus_F")]
    pub bonus_f: String,
    #[astra(key = "@Bond_F")]
    pub bond_f: Option<i32>,
    #[astra(key = "@Border_F")]
    pub border_f: Option<i32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSitUpFallData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@PerfectLimit")]
    pub perfect_limit: Option<f32>,
    #[astra(key = "@GainPower")]
    pub gain_power: Option<f32>,
    #[astra(key = "@FallSpeed")]
    pub fall_speed: Option<f32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MusclePushUpSpeedData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@SpeedMin")]
    pub speed_min: Option<f32>,
    #[astra(key = "@SpeedMax")]
    pub speed_max: Option<f32>,
    #[astra(key = "@LevelUpCount")]
    pub level_up_count: Option<i32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatJudgeAreaData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@BadFrameFirst")]
    pub bad_frame_first: Option<f32>,
    #[astra(key = "@GoodFrameFirst")]
    pub good_frame_first: Option<f32>,
    #[astra(key = "@PerfectFrameFirst")]
    pub perfect_frame_first: Option<f32>,
    #[astra(key = "@PerfectFrameLatter")]
    pub perfect_frame_latter: Option<f32>,
    #[astra(key = "@GoodFrameLatter")]
    pub good_frame_latter: Option<f32>,
    #[astra(key = "@BadFrameLatter")]
    pub bad_frame_latter: Option<f32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatScoreListData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@UseCount")]
    pub use_count: Option<i32>,
    #[astra(key = "@IsDoubleChoice")]
    pub is_double_choice: Option<bool>,
    #[astra(key = "@Speed")]
    pub speed: Option<f32>,
    #[astra(key = "@Length")]
    pub length: Option<f32>,
    #[astra(key = "@Type_1")]
    pub type_1: String,
    #[astra(key = "@LotteryParam_1")]
    pub lottery_param_1: Option<f32>,
    #[astra(key = "@Type_2")]
    pub type_2: String,
    #[astra(key = "@LotteryParam_2")]
    pub lottery_param_2: Option<f32>,
    #[astra(key = "@Type_3")]
    pub type_3: String,
    #[astra(key = "@LotteryParam_3")]
    pub lottery_param_3: Option<f32>,
    #[astra(key = "@Type_4")]
    pub type_4: String,
    #[astra(key = "@LotteryParam_4")]
    pub lottery_param_4: Option<f32>,
    #[astra(key = "@Type_5")]
    pub type_5: String,
    #[astra(key = "@LotteryParam_5")]
    pub lottery_param_5: Option<f32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleSquatMusicSheet {
    #[astra(key = "@ID", public_array)]
    pub id: String,
    #[astra(key = "@TypeA_L")]
    pub type_a_l: Option<i8>,
    #[astra(key = "@TypeA_R")]
    pub type_a_r: Option<i8>,
    #[astra(key = "@TypeB_L")]
    pub type_b_l: Option<i8>,
    #[astra(key = "@TypeB_R")]
    pub type_b_r: Option<i8>,
    #[astra(key = "@TypeC_L")]
    pub type_c_l: Option<i8>,
    #[astra(key = "@TypeC_R")]
    pub type_c_r: Option<i8>,
    #[astra(key = "@TypeD_L")]
    pub type_d_l: Option<i8>,
    #[astra(key = "@TypeD_R")]
    pub type_d_r: Option<i8>,
    #[astra(key = "@TypeE_L")]
    pub type_e_l: Option<i8>,
    #[astra(key = "@TypeE_R")]
    pub type_e_r: Option<i8>,
    #[astra(key = "@Ensure")]
    pub ensure: Option<i32>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct MuscleAssistData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Level_00")]
    pub level_00: Option<i32>,
    #[astra(key = "@Level_01")]
    pub level_01: Option<i32>,
    #[astra(key = "@Level_02")]
    pub level_02: Option<i32>,
    #[astra(key = "@Level_03")]
    pub level_03: Option<i32>,
    #[astra(key = "@Level_04")]
    pub level_04: Option<i32>,
    #[astra(key = "@Level_05")]
    pub level_05: Option<i32>,
    #[astra(key = "@Level_06")]
    pub level_06: Option<i32>,
    #[astra(key = "@Level_07")]
    pub level_07: Option<i32>,
    #[astra(key = "@Level_08")]
    pub level_08: Option<i32>,
    #[astra(key = "@Level_09")]
    pub level_09: Option<i32>,
    #[astra(key = "@Level_10")]
    pub level_10: Option<i32>,
}