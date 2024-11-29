use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: RecordId,
    pub name: String,
    pub age: u8,
}
