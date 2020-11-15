use warp::{Filter, http::StatusCode, test::request};

#[tokio::test]
async fn oembed() {
   let data = crate::models::embeds::Oembed {
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
