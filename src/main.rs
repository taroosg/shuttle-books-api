use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

async fn list(State(state): State<MyState>) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&state.pool)
        .await
    {
        Ok(books) => Ok((StatusCode::OK, Json(books))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(book) => Ok((StatusCode::OK, Json(book))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add(
    State(state): State<MyState>,
    Json(data): Json<BookNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, isbn) VALUES ($1, $2) RETURNING id, title, isbn",
    )
    .bind(&data.title)
    .bind(&data.isbn)
    .fetch_one(&state.pool)
    .await
    {
        Ok(book) => Ok((StatusCode::CREATED, Json(book))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/books", get(list))
        // .route("/books", post(add))
        .route("/books/:id", get(retrieve))
        .with_state(state);

    Ok(router.into())
}

#[derive(Deserialize)]
struct BookNew {
    pub title: String,
    pub isbn: String,
}

#[derive(Serialize, FromRow)]
struct Book {
    pub id: i32,
    pub title: String,
    pub isbn: String,
}
