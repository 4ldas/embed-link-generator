use warp::{http::StatusCode, test::request};


#[tokio::test]
async fn wrong_color_length() {
    let data = crate::models::embeds::Embed {
        title: None,
        description: None,
        site_name: None,
        image: None,
        color: Some(String::from("test")),
        etype: None,
        author_name: None,
        author_url: None,
        provider_name: None,
        provider_url: None
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
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
