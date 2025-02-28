use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AnimalBook {
    pub animals: Sheet<IndexMap<String, AnimalData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AnimalData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@ANID", id)]
    pub anid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@IconName")]
    pub icon_name: String,
    #[astra(key = "@Category")]
    pub category: String,
    #[astra(key = "@Radius")]
    pub radius: f32,
    #[astra(key = "@NID")]
    pub nid: String,
    #[astra(key = "@PID")]
    pub pid: String,
    #[astra(key = "@Item")]
    pub item: String,
    #[astra(key = "@Rare")]
    pub rare: bool,
    #[astra(key = "@M001")]
    pub m_001: bool,
    #[astra(key = "@M002")]
    pub m_002: bool,
    #[astra(key = "@M003")]
    pub m_003: bool,
    #[astra(key = "@M004")]
    pub m_004: bool,
    #[astra(key = "@M005")]
    pub m_005: bool,
    #[astra(key = "@M006")]
    pub m_006: bool,
    #[astra(key = "@M007")]
    pub m_007: bool,
    #[astra(key = "@M008")]
    pub m_008: bool,
    #[astra(key = "@M009")]
    pub m_009: bool,
    #[astra(key = "@M010")]
    pub m_010: bool,
    #[astra(key = "@M011")]
    pub m_011: bool,
    #[astra(key = "@M012")]
    pub m_012: bool,
    #[astra(key = "@M013")]
    pub m_013: bool,
    #[astra(key = "@M014")]
    pub m_014: bool,
    #[astra(key = "@M015")]
    pub m_015: bool,
    #[astra(key = "@M016")]
    pub m_016: bool,
    #[astra(key = "@M017")]
    pub m_017: bool,
    #[astra(key = "@M018")]
    pub m_018: bool,
    #[astra(key = "@M019")]
    pub m_019: bool,
    #[astra(key = "@M020")]
    pub m_020: bool,
    #[astra(key = "@M021")]
    pub m_021: bool,
    #[astra(key = "@M022")]
    pub m_022: bool,
    #[astra(key = "@M023")]
    pub m_023: bool,
    #[astra(key = "@M024")]
    pub m_024: bool,
    #[astra(key = "@M025")]
    pub m_025: bool,
    #[astra(key = "@M026")]
    pub m_026: bool,
    #[astra(key = "@S001")]
    pub s_001: bool,
    #[astra(key = "@S002")]
    pub s_002: bool,
    #[astra(key = "@S003")]
    pub s_003: bool,
    #[astra(key = "@S004")]
    pub s_004: bool,
    #[astra(key = "@S005")]
    pub s_005: bool,
    #[astra(key = "@S006")]
    pub s_006: bool,
    #[astra(key = "@S007")]
    pub s_007: bool,
    #[astra(key = "@S008")]
    pub s_008: bool,
    #[astra(key = "@S009")]
    pub s_009: bool,
    #[astra(key = "@S010")]
    pub s_010: bool,
    #[astra(key = "@S011")]
    pub s_011: bool,
    #[astra(key = "@S012")]
    pub s_012: bool,
    #[astra(key = "@S013")]
    pub s_013: bool,
    #[astra(key = "@S014")]
    pub s_014: bool,
    #[astra(key = "@S015")]
    pub s_015: bool,
    #[astra(key = "@G001")]
    pub g_001: bool,
    #[astra(key = "@G002")]
    pub g_002: bool,
    #[astra(key = "@G003")]
    pub g_003: bool,
    #[astra(key = "@G004")]
    pub g_004: bool,
    #[astra(key = "@G005")]
    pub g_005: bool,
    #[astra(key = "@G006")]
    pub g_006: bool,
    #[astra(key = "@E001")]
    pub e_001: bool,
    #[astra(key = "@E002")]
    pub e_002: bool,
    #[astra(key = "@E003")]
    pub e_003: bool,
    #[astra(key = "@E004")]
    pub e_004: bool,
    #[astra(key = "@E005")]
    pub e_005: bool,
    #[astra(key = "@E006")]
    pub e_006: bool,
}
