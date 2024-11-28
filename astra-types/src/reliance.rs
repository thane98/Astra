use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct RelianceBook {
    pub reliance_data: Sheet<IndexMap<String, RelianceData>>,
    pub reliance_exp_data: Sheet<IndexMap<String, RelianceExpData>>,
    pub relianace_bonus_data: Sheet<IndexMap<String, Vec<RelianceBonusData>>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Pid", id)]
    pub pid: String,
    #[astra(key = "@ExpType0")]
    pub exp_type_0: u8,
    #[astra(key = "@ExpType1")]
    pub exp_type_1: u8,
    #[astra(key = "@ExpType2")]
    pub exp_type_2: u8,
    #[astra(key = "@ExpType3")]
    pub exp_type_3: u8,
    #[astra(key = "@ExpType4")]
    pub exp_type_4: u8,
    #[astra(key = "@ExpType5")]
    pub exp_type_5: u8,
    #[astra(key = "@ExpType6")]
    pub exp_type_6: u8,
    #[astra(key = "@ExpType7")]
    pub exp_type_7: u8,
    #[astra(key = "@ExpType8")]
    pub exp_type_8: u8,
    #[astra(key = "@ExpType9")]
    pub exp_type_9: u8,
    #[astra(key = "@ExpType10")]
    pub exp_type_10: u8,
    #[astra(key = "@ExpType11")]
    pub exp_type_11: u8,
    #[astra(key = "@ExpType12")]
    pub exp_type_12: u8,
    #[astra(key = "@ExpType13")]
    pub exp_type_13: u8,
    #[astra(key = "@ExpType14")]
    pub exp_type_14: u8,
    #[astra(key = "@ExpType15")]
    pub exp_type_15: u8,
    #[astra(key = "@ExpType16")]
    pub exp_type_16: u8,
    #[astra(key = "@ExpType17")]
    pub exp_type_17: u8,
    #[astra(key = "@ExpType18")]
    pub exp_type_18: u8,
    #[astra(key = "@ExpType19")]
    pub exp_type_19: u8,
    #[astra(key = "@ExpType20")]
    pub exp_type_20: u8,
    #[astra(key = "@ExpType21")]
    pub exp_type_21: u8,
    #[astra(key = "@ExpType22")]
    pub exp_type_22: u8,
    #[astra(key = "@ExpType23")]
    pub exp_type_23: u8,
    #[astra(key = "@ExpType24")]
    pub exp_type_24: u8,
    #[astra(key = "@ExpType25")]
    pub exp_type_25: u8,
    #[astra(key = "@ExpType26")]
    pub exp_type_26: u8,
    #[astra(key = "@ExpType27")]
    pub exp_type_27: u8,
    #[astra(key = "@ExpType28")]
    pub exp_type_28: u8,
    #[astra(key = "@ExpType29")]
    pub exp_type_29: u8,
    #[astra(key = "@ExpType30")]
    pub exp_type_30: u8,
    #[astra(key = "@ExpType31")]
    pub exp_type_31: u8,
    #[astra(key = "@ExpType32")]
    pub exp_type_32: u8,
    #[astra(key = "@ExpType33")]
    pub exp_type_33: u8,
    #[astra(key = "@ExpType34")]
    pub exp_type_34: u8,
    #[astra(key = "@ExpType35")]
    pub exp_type_35: u8,
    #[astra(key = "@ExpType36")]
    pub exp_type_36: u8,
    #[astra(key = "@ExpType37")]
    pub exp_type_37: u8,
    #[astra(key = "@ExpType38")]
    pub exp_type_38: u8,
    #[astra(key = "@ExpType39")]
    pub exp_type_39: u8,
    #[astra(key = "@ExpType40")]
    pub exp_type_40: u8,
    #[astra(key = "@ExpType41")]
    pub exp_type_41: u8,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceExpData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rexid", id)]
    pub rexid: String,
    #[astra(key = "@ExpC")]
    pub exp_c: u8,
    #[astra(key = "@ExpB")]
    pub exp_b: u8,
    #[astra(key = "@ExpA")]
    pub exp_a: u8,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceBonusData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@Hit")]
    pub hit: i8,
    #[astra(key = "@Critical")]
    pub critical: i8,
    #[astra(key = "@Avoid")]
    pub avoid: i8,
    #[astra(key = "@Secure")]
    pub secure: i8,
}
