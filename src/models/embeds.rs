use serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Embed {
    //HTML
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub image: Option<String>,
    pub color: Option<String>,
    //oEmbed
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub etype: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
}


#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Oembed {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub etype: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>
}

