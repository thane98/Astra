use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AnimSetBook {
    pub sets: Sheet<IndexMap<String, AnimSet>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AnimSet {
    #[astra(key = "@Name", id)]
    pub name: String,
    #[astra(key = "@Attack1")]
    pub attack_1: String,
    #[astra(key = "@Attack2")]
    pub attack_2: String,
    #[astra(key = "@Attack3")]
    pub attack_3: String,
    #[astra(key = "@Attack4")]
    pub attack_4: String,
    #[astra(key = "@Attack5")]
    pub attack_5: String,
    #[astra(key = "@AttackC")]
    pub attack_c: String,
    #[astra(key = "@AttackT")]
    pub attack_t: String,
    #[astra(key = "@DamageHigh")]
    pub damage_high: String,
    #[astra(key = "@DamageMidB")]
    pub damage_mid_b: String,
    #[astra(key = "@DamageMidDU")]
    pub damage_mid_du: String,
    #[astra(key = "@DamageMidUD")]
    pub damage_mid_ud: String,
    #[astra(key = "@DieB")]
    pub die_b: String,
    #[astra(key = "@DieL")]
    pub die_l: String,
    #[astra(key = "@DieR")]
    pub die_r: String,
    #[astra(key = "@Dive")]
    pub dive: String,
    #[astra(key = "@Engage1")]
    pub engage_1: String,
    #[astra(key = "@Engage2")]
    pub engage_2: String,
    #[astra(key = "@Engage3")]
    pub engage_3: String,
    #[astra(key = "@EvasionB")]
    pub evasion_b: String,
    #[astra(key = "@EvasionL")]
    pub evasion_l: String,
    #[astra(key = "@EvasionR")]
    pub evasion_r: String,
    #[astra(key = "@Guard")]
    pub guard: String,
    #[astra(key = "@HoveringLoop")]
    pub hovering_loop: String,
    #[astra(key = "@IdleDying")]
    pub idle_dying: String,
    #[astra(key = "@IdleNormal")]
    pub idle_normal: String,
    #[astra(key = "@ParryL")]
    pub parry_l: String,
    #[astra(key = "@ParryR")]
    pub parry_r: String,
    #[astra(key = "@Ready")]
    pub ready: String,
    #[astra(key = "@RelaxLoop")]
    pub relax_loop: String,
    #[astra(key = "@Repelled")]
    pub repelled: String,
    #[astra(key = "@RunLoop")]
    pub run_loop: String,
    #[astra(key = "@RunStart")]
    pub run_start: String,
    #[astra(key = "@Special1")]
    pub special_1: String,
    #[astra(key = "@Start")]
    pub start: String,
    #[astra(key = "@Win")]
    pub win: String,
    #[astra(key = "@WinLoop")]
    pub win_loop: String,
}
