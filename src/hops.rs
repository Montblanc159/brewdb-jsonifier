use libsql::{Connection, de};

const DB_QUERY: &str = "SELECT * FROM hop";

#[derive(Debug, serde::Deserialize)]
pub struct Hop {
    pub Id: Option<i64>,
    pub BrewingUsage: Option<i64>,
    pub Name: Option<String>,
    pub Aroma: Option<String>,
    pub Pedigree: Option<String>,
    pub AlphaMax: Option<f32>,
    pub AlphaMin: Option<f32>,
    pub BetaMax: Option<f32>,
    pub BetaMin: Option<f32>,
    pub CoHumuloneMax: Option<f32>,
    pub CoHumuloneMin: Option<f32>,
    pub Info: Option<String>,
    pub Styles: Option<String>,
    pub TotalOilMax: Option<f32>,
    pub TotalOilMin: Option<f32>,
    pub Trade: Option<String>,
}

pub async fn query(conn: Connection) -> Vec<Hop> {
    let mut statement = conn.prepare(DB_QUERY).await.unwrap();
    let mut rows = statement.query([1]).await.unwrap();
    let mut hops = vec![];

    while let Some(row) = rows.next().await.unwrap() {
        hops.push(de::from_row::<Hop>(&row).unwrap());
    }

    hops
}
