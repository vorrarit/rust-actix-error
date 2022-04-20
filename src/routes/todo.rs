use std::fmt::Debug;
use actix_web::{post, HttpResponse, ResponseError};
use crate::services::todo_service::{TodoService, TodoServiceError};

#[tracing::instrument(
    name = "add todo"
)]
#[post("/todo")]
pub async fn todo(body:String) -> Result<HttpResponse, WebError> {
    tracing::info!("{}", format!("description {}", body.as_str()));
    let res = TodoService::add(body.as_str())?;

    Ok(HttpResponse::Ok().body(res))
}

#[derive(thiserror::Error)]
pub enum WebError {
    #[error("{0}")]
    TodoRouteError(#[from] TodoServiceError)
}

impl Debug for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_format(self, f)
    }
}

impl ResponseError for WebError {}

fn error_chain_format(e: &impl std::error::Error, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}'n", e)?;

    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}