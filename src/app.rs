use axum::{
    extract::{Path, State},
    response::Html,
    Form,
};
use leptos::{ssr::render_to_string, *};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{
    errorresponse::AppResult,
    state::{SharedState, Todo},
};

#[component]
fn Todo(todo: Todo) -> impl IntoView {
    view! {
        <li class="w-full flex h-10">
            <input
                type="checkbox"
                class="checkbox checkbox-primary"
                hx-trigger="click"
                hx-post=format!("/check/{}", todo.id)
                hx-target="closest li"
                hx-swap="outerHTML"
                checked=todo.is_completed
            />

            <span class="grow">{todo.title}</span>

            <button
                hx-delete=format!("/delete/{}", todo.id)
                hx-target="closest li"
                hx-swap="delete"
                aria-label="delete"
            >
                <svg
                    aria-hidden="true"
                    focusable="false"
                    width="17"
                    height="17"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="m.967 14.217 5.8-5.906-5.765-5.89L3.094.26l5.783 5.888L14.66.26l2.092 2.162-5.766 5.889 5.801 5.906-2.092 2.162-5.818-5.924-5.818 5.924-2.092-2.162Z"
                        fill="#000"
                    ></path>
                </svg>
            </button>
        </li>
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
                <head>
                    <title>todo app</title>
                    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
                    <link href="./output.css" rel="stylesheet"/>
                </head>
                <body class="grid place-items-center">
                    <main>
                        <h1 class="text-6xl">Todo app</h1>
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
                </body>
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
) -> AppResult<Html<String>> {
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
