use axum::Json;
use axum::response::{self, IntoResponse};
use serde::{Deserialize, Serialize};
use crate::utils::response::status_code::StatusCode;

pub mod status_code;

pub type Result<T> = core::result::Result<T, StatusCode>;

#[derive(Serialize, Deserialize)]
pub struct Response<T>
{
    pub code: u32,
    pub msg: &'static str,
    pub data: Option<T>,
}

pub fn from_status_code(status: StatusCode) -> response::Response
{
    Json(Response {
        code: status.0,
        msg: status.1,
        data: None::<()>,
    }).into_response()
}

impl Default for Response<()> {
    fn default() -> Self {
        Response {
            code: StatusCode::OK.0,
            msg: StatusCode::OK.1,
            data: None,
        }
    }
}

#[allow(unused)]
pub fn success_without_data() -> response::Response {
    Json(Response::default()).into_response()
}

pub fn success<T>(data: Option<T>) -> response::Response
    where
        T: Serialize,
{
    Json(Response {
        code: StatusCode::OK.0,
        msg: StatusCode::OK.1,
        data,
    }).into_response()
}

pub fn error(status: StatusCode) -> response::Response
{
    from_status_code(status)
}