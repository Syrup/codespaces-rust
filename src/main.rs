mod models;

use models::{Person, Record};
use std::sync::LazyLock;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("dono-01.danbot.host:9670").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    // let mut query = db.query("SELECT * FROM person").await?;

    let persons: Vec<Record> = db.select("person").await?;

    dbg!(&persons);

    Ok(())
}

async fn connect() -> surrealdb::Result<()> {
    DB.connect::<Ws>("dono-01.danbot.host:9670").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("test").use_db("test").await?;

    Ok(())
}

#[tokio::test]
async fn test_random_float() -> surrealdb::Result<()> {
    connect().await?;

    let randfloat: f64 = DB.run("rand::float").await?;

    dbg!(randfloat);

    Ok(())
}

#[tokio::test]
async fn test_log_persons() {
    connect().await.unwrap();

    let mut persons: Vec<Record> = DB.select("person").await.unwrap();

    persons.sort_by(|a, b| a.name.cmp(&b.name));

    for (i, person) in persons.iter().enumerate() {
        println!(
            "------{}------\nID: {}\nName: {}\nAge: {}",
            i + 1,
            person.id.to_string(),
            person.name,
            person.age
        );
    }
}

#[tokio::test]
async fn test_query() -> surrealdb::Result<()> {
    connect().await.unwrap();

    let mut query_result = DB.query("SELECT * FROM person").await?;

    let people: Vec<Record> = query_result.take(0)?;

    dbg!(people);

    Ok(())
}
