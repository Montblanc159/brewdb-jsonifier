pub const DB_QUERY: &str = "SELECT \
    m.Id AS id, \
    m.Name AS name, \
    m.Description AS description, \
    CAST(m.EBCMin AS FLOAT) AS ebc_min, \
    CAST(m.EBCMax AS FLOAT) AS ebc_max, \
    ms.Name AS maltster, \
    m.Ratio AS ratio, \
    m.Yield AS grain_yield, \
    m.Moisture AS moisture, \
    m.DiastaticPower AS diastatic_power, \
    m.KolbachIndex AS kolbach_index, \
    m.TotalNitrogen AS total_nitrogen, \
    m.TotalProtein AS total_protein \
    FROM malt m \
    JOIN lookupmaltster ms ON ms.id = m.Maltster
    ORDER BY m.Name ASC";

pub const FILE_PATH: &str = "malts.json";

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Malt {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub ebc_min: Option<f32>,
    pub ebc_max: Option<f32>,
    pub maltster: Option<String>,
    pub ratio: Option<u8>,
    pub grain_yield: Option<f32>,
    pub moisture: Option<f32>,
    pub diastatic_power: Option<u32>,
    pub kolbach_index: Option<u8>,
    pub total_nitrogen: Option<f32>,
    pub total_protein: Option<f32>,
    pub snr: Option<u8>,
}

impl super::Ingredient<Malt> {}
