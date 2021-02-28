use warp::{http::StatusCode, test::request};


#[tokio::test]
async fn oembed() {
    let data = crate::models::embeds::Embed {
        title: None,
        description: None,
        site_name: None,
        color: None,
        image: None,
        etype: None,
        author_name: Some(String::from("author")),
        author_url: Some(String::from("https://example.org")),
        provider_name: Some(String::from("provider")),
        provider_url: Some(String::from("https://example.org"))
    };
    let api = crate::routes::embeds::oembed();

    assert_eq!(
        request()
            .method("GET")
            .path(&format!("/oembed?{}", serde_urlencoded::to_string(&data).unwrap()))
            .reply(&api)
            .await.status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn embed() {
    let data = crate::models::embeds::Embed {
        title: Some(String::from("test")),
        description: None,
        site_name: None,
        image: None,
        color: None,
        etype: None,
        author_name: Some(String::from("author")),
        author_url: Some(String::from("https://example.org")),
        provider_name: Some(String::from("provider")),
        provider_url: Some(String::from("https://example.org"))
    };
    let config: crate::models::config::Config = toml::from_str(&std::fs::read_to_string(std::env::var("CONFIG_FILE")
            .unwrap_or("config.toml".to_string()))
        .expect("failed to open the toml file"))
        .expect("failed to parse the toml file");
    let config = std::sync::Arc::new(config);
    let api = crate::routes::embeds::embed(config);

    assert_eq!(
        request()
            .method("GET")
            .path(&format!("/embed?{}", serde_urlencoded::to_string(&data).unwrap()))
            .reply(&api)
            .await.status(),
        StatusCode::OK
    );
}
