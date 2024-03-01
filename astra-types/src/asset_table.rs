use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AssetTableBook {
    pub asset_defs: Sheet<Vec<AssetDef>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AssetDef {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PresetName")]
    pub preset_name: String,
    #[astra(key = "@Mode")]
    pub mode: Option<i8>,
    #[astra(key = "@Conditions")]
    pub conditions: Vec<String>,
    #[astra(key = "@BodyModel")]
    pub body_model: String,
    #[astra(key = "@DressModel")]
    pub dress_model: String,
    #[astra(key = "@MaskColor100R")]
    pub mask_color_100_r: Option<u8>,
    #[astra(key = "@MaskColor100G")]
    pub mask_color_100_g: Option<u8>,
    #[astra(key = "@MaskColor100B")]
    pub mask_color_100_b: Option<u8>,
    #[astra(key = "@MaskColor075R")]
    pub mask_color_075_r: Option<u8>,
    #[astra(key = "@MaskColor075G")]
    pub mask_color_075_g: Option<u8>,
    #[astra(key = "@MaskColor075B")]
    pub mask_color_075_b: Option<u8>,
    #[astra(key = "@MaskColor050R")]
    pub mask_color_050_r: Option<u8>,
    #[astra(key = "@MaskColor050G")]
    pub mask_color_050_g: Option<u8>,
    #[astra(key = "@MaskColor050B")]
    pub mask_color_050_b: Option<u8>,
    #[astra(key = "@MaskColor025R")]
    pub mask_color_025_r: Option<u8>,
    #[astra(key = "@MaskColor025G")]
    pub mask_color_025_g: Option<u8>,
    #[astra(key = "@MaskColor025B")]
    pub mask_color_025_b: Option<u8>,
    #[astra(key = "@HeadModel")]
    pub head_model: String,
    #[astra(key = "@HairModel")]
    pub hair_model: String,
    #[astra(key = "@HairR")]
    pub hair_r: Option<u8>,
    #[astra(key = "@HairG")]
    pub hair_g: Option<u8>,
    #[astra(key = "@HairB")]
    pub hair_b: Option<u8>,
    #[astra(key = "@GradR")]
    pub grad_r: Option<u8>,
    #[astra(key = "@GradG")]
    pub grad_g: Option<u8>,
    #[astra(key = "@GradB")]
    pub grad_b: Option<u8>,
    #[astra(key = "@SkinR")]
    pub skin_r: Option<u8>,
    #[astra(key = "@SkinG")]
    pub skin_g: Option<u8>,
    #[astra(key = "@SkinB")]
    pub skin_b: Option<u8>,
    #[astra(key = "@ToonR")]
    pub toon_r: Option<u8>,
    #[astra(key = "@ToonG")]
    pub toon_g: Option<u8>,
    #[astra(key = "@ToonB")]
    pub toon_b: Option<u8>,
    #[astra(key = "@RideModel")]
    pub ride_model: String,
    #[astra(key = "@RideDressModel")]
    pub ride_dress_model: String,
    #[astra(key = "@LeftHand")]
    pub left_hand: String,
    #[astra(key = "@RightHand")]
    pub right_hand: String,
    #[astra(key = "@Trail")]
    pub trail: String,
    #[astra(key = "@Magic")]
    pub magic: String,
    #[astra(key = "@Acc1.Locator")]
    pub acc_1_locator: String,
    #[astra(key = "@Acc1.Model")]
    pub acc_1_model: String,
    #[astra(key = "@Acc2.Locator")]
    pub acc_2_locator: String,
    #[astra(key = "@Acc2.Model")]
    pub acc_2_model: String,
    #[astra(key = "@Acc3.Locator")]
    pub acc_3_locator: String,
    #[astra(key = "@Acc3.Model")]
    pub acc_3_model: String,
    #[astra(key = "@Acc4.Locator")]
    pub acc_4_locator: String,
    #[astra(key = "@Acc4.Model")]
    pub acc_4_model: String,
    #[astra(key = "@Acc5.Locator")]
    pub acc_5_locator: String,
    #[astra(key = "@Acc5.Model")]
    pub acc_5_model: String,
    #[astra(key = "@Acc6.Locator")]
    pub acc_6_locator: String,
    #[astra(key = "@Acc6.Model")]
    pub acc_6_model: String,
    #[astra(key = "@Acc7.Locator")]
    pub acc_7_locator: String,
    #[astra(key = "@Acc7.Model")]
    pub acc_7_model: String,
    #[astra(key = "@Acc8.Locator")]
    pub acc_8_locator: String,
    #[astra(key = "@Acc8.Model")]
    pub acc_8_model: String,
    #[astra(key = "@BodyAnim")]
    pub body_anim: String,
    #[astra(key = "@InfoAnim")]
    pub info_anim: String,
    #[astra(key = "@TalkAnim")]
    pub talk_anim: String,
    #[astra(key = "@DemoAnim")]
    pub demo_anim: String,
    #[astra(key = "@HubAnim")]
    pub hub_anim: String,
    #[astra(key = "@ScaleAll")]
    pub scale_all: Option<f32>,
    #[astra(key = "@ScaleHead")]
    pub scale_head: Option<f32>,
    #[astra(key = "@ScaleNeck")]
    pub scale_neck: Option<f32>,
    #[astra(key = "@ScaleTorso")]
    pub scale_torso: Option<f32>,
    #[astra(key = "@ScaleShoulders")]
    pub scale_shoulders: Option<f32>,
    #[astra(key = "@ScaleArms")]
    pub scale_arms: Option<f32>,
    #[astra(key = "@ScaleHands")]
    pub scale_hands: Option<f32>,
    #[astra(key = "@ScaleLegs")]
    pub scale_legs: Option<f32>,
    #[astra(key = "@ScaleFeet")]
    pub scale_feet: Option<f32>,
    #[astra(key = "@VolumeArms")]
    pub volume_arms: Option<f32>,
    #[astra(key = "@VolumeLegs")]
    pub volume_legs: Option<f32>,
    #[astra(key = "@VolumeBust")]
    pub volume_bust: Option<f32>,
    #[astra(key = "@VolumeAbdomen")]
    pub volume_abdomen: Option<f32>,
    #[astra(key = "@VolumeTorso")]
    pub volume_torso: Option<f32>,
    #[astra(key = "@VolumeScaleArms")]
    pub volume_scale_arms: Option<f32>,
    #[astra(key = "@VolumeScaleLegs")]
    pub volume_scale_legs: Option<f32>,
    #[astra(key = "@MapScaleAll")]
    pub map_scale_all: Option<f32>,
    #[astra(key = "@MapScaleHead")]
    pub map_scale_head: Option<f32>,
    #[astra(key = "@MapScaleWing")]
    pub map_scale_wing: Option<f32>,
    #[astra(key = "@Voice")]
    pub voice: String,
    #[astra(key = "@FootStep")]
    pub foot_step: String,
    #[astra(key = "@Material")]
    pub material: String,
    #[astra(key = "@Comment")]
    pub comment: String,
}
