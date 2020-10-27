use crate::handlers::{errors::handle_rejection, embeds as handlers};
use crate::models::embeds as models;
use crate::models::config::Config;
use warp::Filter;

use std::sync::Arc;

pub fn embeds(config: Arc<Config>) -> impl Filter<Extract=impl warp::Reply, Error=std::convert::Infallible> + Clone {
    oembed().or(embed(config)).recover(handle_rejection)
}

pub fn oembed() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("oembed")
        .and(warp::get())
        .and(warp::query::<models::Oembed>())
        .and_then(handlers::oembed)
}

pub fn embed(config: Arc<Config>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("embed")
        .and(warp::get())
        .and(warp::query::<models::Embed>())
        .and(warp::any().map(move || config.clone()))
        .and_then(handlers::create)
}
