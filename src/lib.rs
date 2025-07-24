mod hops;

use libsql::{Builder, Connection};

const DB_PATH: &str = "submodules/BrewDB/brewDB.sqlite";

async fn connection() -> Connection {
    let db = Builder::new_local(DB_PATH).build().await.unwrap();

    db.connect().unwrap()
}

pub async fn print_result() {
    let connection = connection().await;

    let hops = hops::query(connection).await;

    println!("{:?}", hops)
}
