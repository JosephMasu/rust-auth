use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct  ErrorResponse{
    pub stattus: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage{
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}
impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => format!("Password must not be more than {} characters", max_length),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(),
            ErrorMessage::PermissionDenied => "You are not allowed to perform this action".to_string(),
            ErrorMessage::UserNotAuthenticated => "Authentication required. Please log in.".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError { 
            message: message.into(),
            status}
    }
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError{
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError{
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError{
            status: StatusCode::CONFLICT,
            message: message.into(),
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError{
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }
    pub fn into_http_response(self) -> Response{
        let json_response = Json(ErrorResponse{
            stattus: "fail".to_string(),
            message: self.message.clone(),
        });
        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "HttpError: mesaage: {}, status: {}", 
            self.message, self.status)
    }
}

impl std::error::Error for HttpError {}

pub type Result<T> = std::result::Result<T, HttpError>;