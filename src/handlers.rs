use axum::{
    response::Html,
    extract::{State, Query},
    Json
};
use validator::Validate;
use super::models::{
    Result, AppState, Embed
};


pub async fn embed( State(state): State<AppState>, Query(params): Query<Embed> ) -> Result<Html<String>> {
    params.validate()?;

    Ok(Html(format!(r#"<DOCTYPE html>
<html>
    <head>
        {}{}{}{}{}{}
    </head>
    <body>
        <script>location="/"</script>
    </body>
</html>"#,

        if let Some(t) = &params.title {
            format!(r#"<meta property="og:title" content="{}">
        "#, ammonia::clean(t))
        } else { "".to_string() },
        if let Some(d) = &params.description {
            format!(r#"<meta property="og:description" content="{}">
        "#, ammonia::clean(d))
        } else { "".to_string() },
        if let Some(img) = &params.image {
            format!(r#"<meta property="og:image" content="{}">
        "#, ammonia::clean(img))
        } else { "".to_string() },
        if let Some(sn) = &params.site_name {
            format!(r#"<meta property="og:site_name" content="{}">
        "#, ammonia::clean(sn))
        } else { "".to_string() },
        if let Some(c) = &params.color {
            format!(r##"<meta property="theme-color" content="#{}">
        "##, ammonia::clean(c))
        } else { "".to_string() },

        format!(r#"<link type="application/json+oembed" href="{}/oembed?{}">"#, state.root_url, &serde_urlencoded::to_string(Embed {
            title: None,
            description: None,
            site_name: None,
            image: None,
            color: None,
            type_: params.type_,
            author_name: params.author_name,
            author_url: params.author_url,
            provider_name: params.provider_name,
            provider_url: params.provider_url
            
        })?)
    )))
}

pub async fn oembed( Query(params): Query<Embed> ) -> Result<Json<Embed>> {
    params.validate()?;

    Ok(Json(params))
}