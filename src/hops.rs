pub const DB_QUERY: &str = "SELECT \
    h.Id as id, \
    bu.Name as brewing_usage, \
    h.Name as name, \
    h.Aroma as aroma, \
    h.Pedigree as pedigree, \
    h.AlphaMin as alpha_min, \
    h.AlphaMax as alpha_max, \
    h.BetaMin as beta_min, \
    h.BetaMax as beta_max, \
    h.CoHumuloneMin as cohumumulone_min, \
    h.CoHumuloneMax as cohumumulone_max, \
    h.Info as info, \
    h.Styles as styles, \
    h.TotalOilMin as total_oil_min, \
    h.TotalOilMax as total_oil_max, \
    h.Trade as trade \
    FROM hop h
    JOIN lookupbrewingusage bu ON bu.id = h.BrewingUsage
    ORDER BY h.Name ASC";
pub const FILE_PATH: &str = "hops.json";

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Hop {
    pub id: Option<i64>,
    pub brewing_usage: Option<String>,
    pub name: Option<String>,
    pub aroma: Option<String>,
    pub pedigree: Option<String>,
    pub alpha_min: Option<f32>,
    pub alpha_max: Option<f32>,
    pub beta_min: Option<f32>,
    pub beta_max: Option<f32>,
    pub cohumulone_min: Option<f32>,
    pub cohumulone_max: Option<f32>,
    pub info: Option<String>,
    pub styles: Option<String>,
    pub total_oil_min: Option<f32>,
    pub total_oil_max: Option<f32>,
    pub trade: Option<String>,
}

impl super::Ingredient<Hop> {}
