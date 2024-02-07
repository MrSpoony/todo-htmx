use axum::response::{Html, IntoResponse, Response};
use http::HeaderValue;
use leptos::{ssr::render_to_string, *};

#[component]
fn ErrorAlert() -> impl IntoView {
    view! {
        <div
            role="alert"
            hx-get="/empty"
            hx-trigger="load delay:5.5s"
            classes="add opacity-0:5s"
            class="alert alert-error transition duration-500"
            hx-swap="outerHTML"
        >
            <svg
                aria-hidden="true"
                focusable="false"
                xmlns="http://www.w3.org/2000/svg"
                class="stroke-current shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                ></path>
            </svg>
            <span>An error occured</span>
        </div>
    }
}

#[component]
fn WarningAlert(error_msg: String) -> impl IntoView {
    view! {
        <div
            role="alert"
            hx-get="/empty"
            hx-trigger="load delay:5.5s"
            classes="add opacity-0:5s"
            class="alert alert-warning transition duration-500"
            hx-swap="outerHTML"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="stroke-current shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                ></path>
            </svg>
            <span>Warning: {error_msg}</span>
        </div>
    }
}

// Make our own error that wraps `anyhow::Error`.
pub enum AppError<T>
where
    T: IntoResponse,
{
    PubErr(T),
    PrivErr(anyhow::Error),
}
pub type AppResult<T, U = ()> = Result<T, AppError<U>>;

// Tell axum how to convert `AppError` into a response.
impl<T> IntoResponse for AppError<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            AppError::PubErr(resp) => {
                let mut resp = resp.into_response();
                resp.headers_mut()
                    .insert("Hx-Retarget", HeaderValue::from_static("#alerts"));
                resp
            }
            AppError::PrivErr(err) => {
                println!("Error: {}", err); // TODO: add proper logging
                let mut resp = Html(
                    render_to_string(move || {
                        view! { <ErrorAlert/> }
                    })
                    .to_string(),
                )
                .into_response();
                resp.headers_mut()
                    .insert("Hx-Retarget", HeaderValue::from_static("#alerts"));
                resp
            }
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<T, E> From<E> for AppError<T>
where
    E: Into<anyhow::Error>,
    T: IntoResponse,
{
    fn from(err: E) -> Self {
        Self::PrivErr(err.into())
    }
}

impl AppError<Html<String>> {
    pub fn new(error_msg: String) -> Self {
        AppError::PubErr(Html(
            render_to_string(move || {
                view! { <WarningAlert error_msg=error_msg/> }
            })
            .to_string(),
        ))
    }
}
