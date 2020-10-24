use warp::{Reply, reject::Rejection, http::StatusCode};
use std::convert::Infallible;
use crate::models::errors;

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    //init
    let code;
    let message;

    //assigning messages to errors
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    }
    else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } 
    else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Deserialization error";
    }
    else if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid query";
    }

    else if let Some(errors::InvalidLength) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Length of some of the parameters are invalid. Make sure they are correct!";
    }

    else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "A unhandled error has occurred, please open a issue or make a merge request on GitHub and we will make sure to fix it as fast as possible!"
    }

    let json = warp::reply::json(&errors::Error{
        code: code.as_u16(),
        message: message.into()
    });

    Ok(warp::reply::with_status(json, code))
}

