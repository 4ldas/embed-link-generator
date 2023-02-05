use serde::{Serialize, Deserialize};
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json
};
use serde_json::json;


pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Clone)]
pub struct AppState {
    pub root_url: String
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    UrlEncoding(#[from] serde_urlencoded::ser::Error)
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Validation => (StatusCode::BAD_REQUEST, "Input validation error"),
            UrlEncoding => (StatusCode::INTERNAL_SERVER_ERROR, "Url encoding error")
        };
        (status, Json(json!({"error": err_msg}))).into_response()
    }
}

#[serde_with::skip_serializing_none]
#[derive(validator::Validate, Serialize, Deserialize)]
pub struct Embed {
    //HTML
    #[validate(length(max = 256))]
    pub title: Option<String>,
    #[validate(length(max = 2048))]
    pub description: Option<String>,
    #[validate(length(max = 256))]
    pub site_name: Option<String>,
    #[validate(length(max = 2048))]
    pub image: Option<String>,
    #[validate(length(equal =6))]
    pub color: Option<String>,
    //oEmbed
    #[validate(length(max = 256))]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[validate(length(max = 256))]
    pub author_name: Option<String>,
    #[validate(length(max = 2048))]
    pub author_url: Option<String>,
    #[validate(length(max = 256))]
    pub provider_name: Option<String>,
    #[validate(length(max = 2048))]
    pub provider_url: Option<String>,
}