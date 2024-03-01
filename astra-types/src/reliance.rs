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
    pub exp_type_0: Option<u8>,
    #[astra(key = "@ExpType1")]
    pub exp_type_1: Option<u8>,
    #[astra(key = "@ExpType2")]
    pub exp_type_2: Option<u8>,
    #[astra(key = "@ExpType3")]
    pub exp_type_3: Option<u8>,
    #[astra(key = "@ExpType4")]
    pub exp_type_4: Option<u8>,
    #[astra(key = "@ExpType5")]
    pub exp_type_5: Option<u8>,
    #[astra(key = "@ExpType6")]
    pub exp_type_6: Option<u8>,
    #[astra(key = "@ExpType7")]
    pub exp_type_7: Option<u8>,
    #[astra(key = "@ExpType8")]
    pub exp_type_8: Option<u8>,
    #[astra(key = "@ExpType9")]
    pub exp_type_9: Option<u8>,
    #[astra(key = "@ExpType10")]
    pub exp_type_10: Option<u8>,
    #[astra(key = "@ExpType11")]
    pub exp_type_11: Option<u8>,
    #[astra(key = "@ExpType12")]
    pub exp_type_12: Option<u8>,
    #[astra(key = "@ExpType13")]
    pub exp_type_13: Option<u8>,
    #[astra(key = "@ExpType14")]
    pub exp_type_14: Option<u8>,
    #[astra(key = "@ExpType15")]
    pub exp_type_15: Option<u8>,
    #[astra(key = "@ExpType16")]
    pub exp_type_16: Option<u8>,
    #[astra(key = "@ExpType17")]
    pub exp_type_17: Option<u8>,
    #[astra(key = "@ExpType18")]
    pub exp_type_18: Option<u8>,
    #[astra(key = "@ExpType19")]
    pub exp_type_19: Option<u8>,
    #[astra(key = "@ExpType20")]
    pub exp_type_20: Option<u8>,
    #[astra(key = "@ExpType21")]
    pub exp_type_21: Option<u8>,
    #[astra(key = "@ExpType22")]
    pub exp_type_22: Option<u8>,
    #[astra(key = "@ExpType23")]
    pub exp_type_23: Option<u8>,
    #[astra(key = "@ExpType24")]
    pub exp_type_24: Option<u8>,
    #[astra(key = "@ExpType25")]
    pub exp_type_25: Option<u8>,
    #[astra(key = "@ExpType26")]
    pub exp_type_26: Option<u8>,
    #[astra(key = "@ExpType27")]
    pub exp_type_27: Option<u8>,
    #[astra(key = "@ExpType28")]
    pub exp_type_28: Option<u8>,
    #[astra(key = "@ExpType29")]
    pub exp_type_29: Option<u8>,
    #[astra(key = "@ExpType30")]
    pub exp_type_30: Option<u8>,
    #[astra(key = "@ExpType31")]
    pub exp_type_31: Option<u8>,
    #[astra(key = "@ExpType32")]
    pub exp_type_32: Option<u8>,
    #[astra(key = "@ExpType33")]
    pub exp_type_33: Option<u8>,
    #[astra(key = "@ExpType34")]
    pub exp_type_34: Option<u8>,
    #[astra(key = "@ExpType35")]
    pub exp_type_35: Option<u8>,
    #[astra(key = "@ExpType36")]
    pub exp_type_36: Option<u8>,
    #[astra(key = "@ExpType37")]
    pub exp_type_37: Option<u8>,
    #[astra(key = "@ExpType38")]
    pub exp_type_38: Option<u8>,
    #[astra(key = "@ExpType39")]
    pub exp_type_39: Option<u8>,
    #[astra(key = "@ExpType40")]
    pub exp_type_40: Option<u8>,
    #[astra(key = "@ExpType41")]
    pub exp_type_41: Option<u8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceExpData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rexid", id)]
    pub rexid: String,
    #[astra(key = "@ExpC")]
    pub exp_c: Option<u8>,
    #[astra(key = "@ExpB")]
    pub exp_b: Option<u8>,
    #[astra(key = "@ExpA")]
    pub exp_a: Option<u8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceBonusData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@Hit")]
    pub hit: Option<i8>,
    #[astra(key = "@Critical")]
    pub critical: Option<i8>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i8>,
    #[astra(key = "@Secure")]
    pub secure: Option<i8>,
}
