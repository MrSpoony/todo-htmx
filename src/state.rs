use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
    pub creation_date: NaiveDateTime,
}

pub type SharedState = State;

#[derive(Debug, Clone)]
pub struct State {
    pub db: Pool<Postgres>,
}

impl State {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { db: pool }
    }
}
