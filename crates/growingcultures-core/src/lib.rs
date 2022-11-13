mod parser;

use url::Url;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InfoNode {
    pub ty: String,
    pub title: String,
    pub items: Vec<Content>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Content {
    Text(String),
    Image(Url, String),
    Audio(Url, String),
    Video(Url, String),
}

impl Content {
    fn from_url_and_title(url: Url, title: String) -> Self {
        let text = || Content::Text(format!("<p><a href=\"{url}\">{title}</a></p>"));

        let Some((_, ext)) = url.path().rsplit_once('.') else {
            return text();
        };
        // TODO: find library to classify extension
        match ext {
            "mp3" | "ogg" => Content::Audio(url, title),
            "mp4" => Content::Video(url, title),
            _ => text(),
        }
    }
}
