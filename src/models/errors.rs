use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub code: u16,
    pub message: String
}

#[derive(Debug)]
pub struct InvalidLength;
impl warp::reject::Reject for InvalidLength {}
