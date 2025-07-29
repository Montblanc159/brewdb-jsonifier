mod hops;
mod malts;
mod yeasts;

use hops::Hop;
use malts::Malt;
use std::fs::File;
use std::io::Write;
use yeasts::Yeast;

use libsql::{Builder, Connection, de};
use serde::{Deserialize, Serialize};

const DB_PATH: &str = "submodules/BrewDB/brewDB.sqlite";

async fn connection() -> Connection {
    let db = Builder::new_local(DB_PATH).build().await.unwrap();

    db.connect().unwrap()
}

pub async fn convert_all_data() {
    let connection = connection().await;

    Ingredient::<Hop>::new(hops::DB_QUERY, hops::FILE_PATH)
        .convert_data(connection.clone())
        .await;

    Ingredient::<Malt>::new(malts::DB_QUERY, malts::FILE_PATH)
        .convert_data(connection.clone())
        .await;

    Ingredient::<Yeast>::new(yeasts::DB_QUERY, yeasts::FILE_PATH)
        .convert_data(connection)
        .await;
}

#[derive(Default)]
struct Ingredient<T> {
    value: Vec<T>,
    query: String,
    file_path: String,
}

impl<T> Ingredient<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Default,
{
    fn new(query: &str, file_path: &str) -> Self {
        Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ..Default::default()
        }
    }
    async fn convert_data(&mut self, conn: Connection) {
        self.value = self.query(conn).await;
        self.write_to_file(self.to_json(&self.value));
    }

    async fn query(&self, conn: Connection) -> Vec<T> {
        let mut statement = conn.prepare(&self.query).await.unwrap();
        let mut rows = statement.query([1]).await.unwrap();
        let mut ingredients = vec![];

        while let Some(row) = rows.next().await.unwrap() {
            ingredients.push(de::from_row::<T>(&row).unwrap());
        }

        ingredients
    }

    fn to_json(&self, object: &Vec<T>) -> String {
        match serde_json::to_string(object) {
            Ok(string) => string,
            Err(err) => {
                panic!("Unable to load data : {err}");
            }
        }
    }

    fn write_to_file(&self, data: String) {
        let mut file = File::create(&self.file_path).expect("Failed to create file");
        match file.write_all(data.as_bytes()) {
            Ok(_) => println!("Ingredients json file created"),
            Err(err) => panic!("Ingredients file creation error {err}"),
        }
    }
}
