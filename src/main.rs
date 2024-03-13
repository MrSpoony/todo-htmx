use axum::routing::{delete, post};
use sqlx::postgres::PgPoolOptions;

use dotenvy::dotenv;

use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use todo_htmx::app::*;
use todo_htmx::fileserv::file_and_error_handler;
use todo_htmx::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv()?;

    let db_connection_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_url)
        .await?;

    let shared_state: SharedState = State::new(pool);

    let app = Router::new()
        .route("/", get(index))
        .route("/check/:id", post(check))
        .route("/add", post(add))
        .route("/delete/:id", delete(delete_todo))
        .route("/empty", get(empty))
        .fallback(file_and_error_handler)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
