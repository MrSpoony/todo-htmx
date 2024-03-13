use axum::response::{Html, IntoResponse, Response};
use http::HeaderValue;
use leptos::{ssr::render_to_string, *};
use tracing::error;

#[component]
fn ErrorAlert() -> impl IntoView {
    view! { <Alert msg="An error occured".to_string() level=AlertLevel::Error/> }
}

#[component]
fn Alert(msg: String, level: AlertLevel) -> impl IntoView {
    view! {
        <div
            role="alert"
            hx-get="/empty"
            hx-trigger="load delay:5.5s"
            classes="add opacity-0:5s"
            class=format!(
                "alert {} transition duration-500",
                match level {
                    AlertLevel::Error => "alert-error",
                    AlertLevel::Warning => "alert-warning",
                    AlertLevel::Info => "alert-info",
                    AlertLevel::Success => "alert-success",
                },
            )

            hx-swap="outerHTML"
        >
            {match level {
                AlertLevel::Error => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            class="stroke-current shrink-0 w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                    }
                }
                AlertLevel::Warning => {
                    view! {
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
                    }
                }
                AlertLevel::Info => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            class="stroke-current shrink-0 w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                    }
                }
                AlertLevel::Success => {
                    view! {
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
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                    }
                }
            }}

            <span>Warning: {msg}</span>
        </div>
    }
}

// Make our own error that wraps `eyre::Error`.
pub enum AppError<T>
where
    T: IntoResponse,
{
    PubErr(T),
    PrivErr(color_eyre::eyre::Error),
}
pub type AppResult<T, U = Html<String>> = Result<T, AppError<U>>;

/// The different levels of errors for alerts.
pub enum AlertLevel {
    Error,
    Warning,
    Info,
    Success,
}

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
                error!("{:?}", err);
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

// This enables using `?` on functions that return `Result<_, color_eyre::eyre::Error>` to turn them into a private error
impl<T, E> From<E> for AppError<T>
where
    E: Into<color_eyre::eyre::Error>,
    T: IntoResponse,
{
    fn from(err: E) -> Self {
        Self::PrivErr(err.into())
    }
}

impl AppError<Html<String>> {
    pub fn new(error_msg: String, level: AlertLevel) -> Self {
        AppError::PubErr(Html(
            render_to_string(move || {
                view! { <Alert msg=error_msg level=level/> }
            })
            .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::ContainsSlice;
    use color_eyre::eyre::eyre;
    use http_body_util::BodyExt;

    use super::*;

    fn get_error_into_response() -> AppResult<String> {
        let err = eyre!("This is some error");
        let res: Result<String, color_eyre::eyre::Error> = Err(err);
        res?;
        Ok("This should not be shown to the public".to_string())
    }

    fn get_public_error_into_response() -> AppResult<String> {
        Err(AppError::new("Input invalid".into(), AlertLevel::Error))
    }

    fn get_successful_into_response() -> AppResult<String> {
        Ok("Success".to_string())
    }

    #[tokio::test]
    async fn private_error_test() {
        let result = get_error_into_response();
        let response = result.into_response();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.contains_slice(b"An error occured"));
        assert!(!body.contains_slice(b"This should not be shown to the public"));
    }

    #[tokio::test]
    async fn public_error_test() {
        let result = get_public_error_into_response();
        let response = result.into_response();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.contains_slice(b"Input invalid"));
    }

    #[tokio::test]
    async fn no_error_test() {
        let result = get_successful_into_response();
        let response = result.into_response();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.contains_slice(b"Success"));
    }
}
