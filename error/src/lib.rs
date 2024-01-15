use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponseError {
    pub error: String,
    pub detail: String,
}

impl AppResponseError {
    pub fn new<S: Into<String>>(error: S, detail: S) -> Self {
        Self {
            error: error.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    LapinError(#[from] lapin::Error),

    // HTTP Errors
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    PermissionDenied(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Unauthorized(String),
}

impl AppError {
    pub fn response(&self) -> AppResponseError {
        AppResponseError {
            error: self.error().to_string(),
            detail: self.to_string(),
        }
    }

    pub fn error(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::PermissionDenied(_) => "PERMISSION_DENIED",
            Self::Conflict(_) => "CONFLICT",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::IoError(_) => "IO_ERROR",
            Self::LapinError(_) => "LAPIN_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::PermissionDenied(_) => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.response())
    }
}
