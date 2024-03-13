use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Form,
};
use http::StatusCode;
use leptos::{ssr::render_to_string, *};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{
    errorresponse::{AlertLevel, AppError, AppResult},
    state::{SharedState, Todo},
};

#[component]
fn Todo(todo: Todo) -> impl IntoView {
    view! {
        <div role="listitem" class="w-full flex items-center">
            <input
                type="checkbox"
                class="checkbox checkbox-primary"
                aria-label="toggle todo"
                hx-trigger="click"
                hx-post=format!("/check/{}", todo.id)
                hx-target="closest div"
                hx-swap="outerHTML"
                checked=todo.is_completed
            />

            <span class="grow px-2">{todo.title}</span>

            <button
                hx-delete=format!("/delete/{}", todo.id)
                hx-target="closest div"
                hx-swap="delete"
                aria-label="delete todo"
            >
                <svg
                    aria-hidden="true"
                    focusable="false"
                    width="17"
                    height="17"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-current shrink-0 h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        d="m.967 14.217 5.8-5.906-5.765-5.89L3.094.26l5.783 5.888L14.66.26l2.092 2.162-5.766 5.889 5.801 5.906-2.092 2.162-5.818-5.924-5.818 5.924-2.092-2.162Z"
                        fill="#000"
                    ></path>
                </svg>
            </button>
        </div>
    }
}

pub async fn index(State(state): State<SharedState>) -> AppResult<Html<String>> {
    let todos = query_as!(
        Todo,
        r#"
        SELECT *
            FROM todo
            ORDER BY creation_date
        "#
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Html(
        render_to_string(move || {
            view! {
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <title>todo app</title>

                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                        <meta
                            name="description"
                            content="This is just a simple todo app to try out new technologies."
                        />

                        <script src="https://unpkg.com/htmx.org@1.9.10"></script>
                        <script src="https://unpkg.com/htmx.org/dist/ext/class-tools.js"></script>

                        <link href="./output.css" rel="stylesheet"/>
                    </head>
                    <body class="grid place-items-center">
                        <main class="prose">
                            <h1>Todo app</h1>
                            <ul>
                                <For
                                    each=move || todos.clone()
                                    key=|todo| todo.id
                                    children=move |todo| {
                                        view! { <Todo todo=todo/> }
                                    }
                                />

                            </ul>
                            <form
                                hx-post="/add"
                                hx-target="ul"
                                hx-swap="beforeend"
                                hx-on:htmx:after-request="this.reset()"
                            >
                                <input
                                    class="input input-bordered"
                                    type="text"
                                    name="title"
                                    placeholder="What needs to be done?"
                                />
                                <button class="btn btn-primary" type="submit">
                                    add
                                </button>
                            </form>
                        </main>
                        <div
                            id="alerts"
                            hx-ext="class-tools"
                            class="absolute bottom-0 right-0 p-5 w-full md:w-96 flex flex-col gap-1"
                        ></div>
                        <div class="opacity-0"></div>
                    </body>
                </html>
            }
        })
        .to_string(),
    ))
}

pub async fn check(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> AppResult<Html<String>> {
    query!(
        r#"
        UPDATE todo
            SET is_completed = NOT is_completed
            WHERE id = $1
        "#,
        id
    )
    .execute(&state.db)
    .await?;
    let todo = query_as!(
        Todo,
        r#"
        SELECT *
            FROM todo
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Html(
        render_to_string(move || {
            view! { <Todo todo=todo.clone()/> }
        })
        .to_string(),
    ))
}

pub async fn delete_todo(State(state): State<SharedState>, Path(id): Path<i32>) -> AppResult<()> {
    query!(
        r#"
        DELETE
            FROM todo
            WHERE id = $1
        "#,
        id
    )
    .execute(&state.db)
    .await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct TodoForm {
    title: String,
}

pub async fn add(
    State(state): State<SharedState>,
    Form(todo_form): Form<TodoForm>,
) -> AppResult<Html<String>, impl IntoResponse> {
    if todo_form.title.trim().is_empty() {
        return Err(AppError::new(
            "Please enter a non-empty todo".to_string(),
            AlertLevel::Warning,
        ));
    }
    // TODO: handle empty input
    let todo = query_as!(
        Todo,
        r#"
        INSERT INTO todo (title, is_completed)
            VALUES ($1, $2) RETURNING *
        "#,
        todo_form.title,
        false
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Html(
        render_to_string(move || {
            view! { <Todo todo=todo/> }
        })
        .to_string(),
    ))
}

pub async fn empty() -> (StatusCode, ()) {
    (StatusCode::OK, ())
}
