use crate::handlers::{errors::handle_rejection, embeds as handlers};
use crate::models::embeds as models;
use warp::Filter;

pub fn embeds() -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    oembed().or(embed()).recover(handle_rejection)
}

pub fn oembed() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("oembed")
        .and(warp::get())
        .and(warp::query::<models::Oembed>())
        .and_then(handlers::oembed)
}

pub fn embed() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("embed")
        .and(warp::get())
        .and(warp::query::<models::Embed>())
        .and_then(handlers::create)
}

