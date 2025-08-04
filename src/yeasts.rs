pub const DB_QUERY: &str = "SELECT \
    y.Id AS id, \
    y.Name AS name,\
    y.Description AS description, \
    y.TempMin AS temp_min, \
    y.TempMax AS temp_max, \
    y.AttenuationMin AS attenuation_min, \
    y.AttenuationMax AS attenuation_max, \
    y.Styles AS styles, \
    y.AlcoholTolerance AS alcohol_tolerance, \
    l.Name AS lab, \
    fl.Name AS flocculation, \
    s.Name AS strain, \
    f.Name AS form \
    FROM yeast y \
    JOIN lookuplab l ON l.Id = y.Lab \
    JOIN lookupyeastflocculation fl ON fl.Id = y.Flocculation \
    JOIN lookupyeaststrain s ON s.Id = y.Strain \
    JOIN lookupyeastform f ON f.Id = y.Form
    ORDER BY y.Name ASC";

pub const FILE_PATH: &str = "yeasts.json";

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Yeast {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub temp_min: Option<u32>,
    pub temp_max: Option<u32>,
    pub lab: Option<String>,
    pub attenuation_min: Option<u32>,
    pub attenuation_max: Option<u32>,
    pub form: Option<String>,
    pub flocculation: Option<String>,
    pub styles: Option<String>,
    pub alcohol_tolerance: Option<f32>,
}

impl super::Ingredient<Yeast> {}
