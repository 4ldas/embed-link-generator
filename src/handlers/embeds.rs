use warp::reject;
use std::sync::Arc;
use crate::models::{config::Config, embeds, errors};

pub async fn create(params: embeds::Embed, config: Arc<Config>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = params.etype.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.author_name.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.author_url.as_ref().filter(|v| v.len() >= 2048) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.provider_name.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.provider_url.as_ref().filter(|v| v.len() >= 2048) { return Err(reject::custom(errors::InvalidLength)) }


    let html = format!(r#"<DOCTYPE html>
<html>
    <head>
        {}{}{}{}{}{}
    </head>
    <body>
        <script>location="/"</script>
    </body>
</html>"#,

        if let Some(t) = &params.title {
            if t.len() < 256 {
            format!(r#"<meta property="og:title" content="{}">
        "#, ammonia::clean(t))
            } else {
                return Err(reject::custom(errors::InvalidLength));
            }
        } else { String::new() },

        if let Some(d) = &params.description {
            if d.len() < 2048 {
                format!(r#"<meta property="og:description" content="{}">
        "#, ammonia::clean(d))
            } else {
                return Err(reject::custom(errors::InvalidLength));
            }
        } else { String::new() },

        if let Some(img) = &params.image {
            if img.len() < 2048 {
                format!(r#"<meta property="og:image" content="{}">
        "#, ammonia::clean(img))
            } else {
                return Err(reject::custom(errors::InvalidLength));
            }
        } else { String::new() },
        
        if let Some(sn) = &params.site_name {
            if sn.len() < 256 {
                format!(r#"<meta property="og:site_name" content="{}">
        "#, ammonia::clean(sn))
            } else {
                return Err(reject::custom(errors::InvalidLength));
            }
        } else { String::new() },

        if let Some(c) = &params.color {
            if c.len() == 6 {
                format!(r##"<meta property="theme-color" content="#{}">
        "##, ammonia::clean(c))
            } else {
                return Err(reject::custom(errors::InvalidLength));
            }
        } else { String::new() },

        format!(r#"<link type="application/json+oembed" href="{}/oembed?{}">"#, config.server.root_url, ammonia::clean(&serde_urlencoded::to_string(embeds::Embed {
            title: None,
            description: None,
            site_name: None,
            image: None,
            color: None,
            etype: params.etype,
            author_name: params.author_name,
            author_url: params.author_url,
            provider_name: params.provider_name,
            provider_url: params.provider_url
            
        }).unwrap())));

    Ok(warp::reply::html(html))
}


pub async fn oembed(params: embeds::Embed) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = params.etype.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.author_name.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.author_url.as_ref().filter(|v| v.len() >= 2048) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.provider_name.as_ref().filter(|v| v.len() >= 256) { return Err(reject::custom(errors::InvalidLength)) };
    if let Some(_) = params.provider_url.as_ref().filter(|v| v.len() >= 2048) { return Err(reject::custom(errors::InvalidLength)) };

    Ok(warp::reply::json(&params))
}
