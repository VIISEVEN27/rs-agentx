use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Media {
    Text(String),
    ImageUrl(String),
    Video(Vec<String>),
    VideoUrl(String),
}

#[derive(Serialize, Deserialize)]
struct Url {
    #[serde(rename = "url")]
    value: String,
}

impl Url {
    fn new(value: String) -> Self {
        Self { value }
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for Media {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            Media::Text(value) => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", value)?;
            }
            Media::ImageUrl(value) => {
                map.serialize_entry("type", "image_url")?;
                map.serialize_entry("image_url", &Url::new(value.clone()))?;
            }
            Media::Video(value) => {
                map.serialize_entry("type", "video")?;
                map.serialize_entry("video", value)?;
            }
            Media::VideoUrl(value) => {
                map.serialize_entry("type", "video_url")?;
                map.serialize_entry("video_url", &Url::new(value.clone()))?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Media {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Type,
            Text,
            ImageUrl,
            Video,
            VideoUrl,
        }

        struct MediaVisitor;

        impl<'de> Visitor<'de> for MediaVisitor {
            type Value = Media;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("enum Media")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut ty: Option<String> = None;
                let mut text: Option<String> = None;
                let mut url: Option<Url> = None;
                let mut urls: Option<Vec<String>> = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Type => {
                            if ty.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            ty = Some(map.next_value()?);
                        }
                        Field::Text => {
                            if text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text = Some(map.next_value()?);
                        }
                        Field::ImageUrl => {
                            if url.is_some() {
                                return Err(serde::de::Error::duplicate_field("image_url"));
                            }
                            url = Some(map.next_value()?);
                        }
                        Field::Video => {
                            if urls.is_some() {
                                return Err(serde::de::Error::duplicate_field("video"));
                            }
                            urls = Some(map.next_value()?);
                        }
                        Field::VideoUrl => {
                            if url.is_some() {
                                return Err(serde::de::Error::duplicate_field("video_url"));
                            }
                            url = Some(map.next_value()?);
                        }
                    }
                }
                let ty = ty.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                match ty.as_str() {
                    "text" => text
                        .map(Media::Text)
                        .ok_or_else(|| serde::de::Error::missing_field("text")),
                    "image_url" => url
                        .map(|url| Media::ImageUrl(url.to_string()))
                        .ok_or_else(|| serde::de::Error::missing_field("image_url")),
                    "video" => urls
                        .map(Media::Video)
                        .ok_or_else(|| serde::de::Error::missing_field("video")),
                    "video_url" => url
                        .map(|url| Media::VideoUrl(url.to_string()))
                        .ok_or_else(|| serde::de::Error::missing_field("video_url")),
                    _ => Err(serde::de::Error::unknown_variant(
                        ty.as_str(),
                        &["image_url", "video", "video_url"],
                    )),
                }
            }
        }
        deserializer.deserialize_map(MediaVisitor)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Message {
    Text { role: Role, content: String },
    Media { role: Role, content: Vec<Media> },
}

impl Message {
    pub fn system(content: String) -> Self {
        Message::Text {
            role: Role::System,
            content,
        }
    }

    pub fn user(content: String) -> Self {
        Message::Text {
            role: Role::User,
            content,
        }
    }

    pub fn assistant(content: String) -> Self {
        Message::Text {
            role: Role::Assistant,
            content,
        }
    }

    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

pub struct MessageBuilder(Vec<Media>);

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder(Vec::new())
    }

    pub fn text(mut self, content: String) -> Self {
        self.0.push(Media::Text(content));
        self
    }

    pub fn image_url(mut self, url: String) -> Self {
        self.0.push(Media::ImageUrl(url));
        self
    }

    pub fn video(mut self, urls: Vec<String>) -> Self {
        self.0.push(Media::Video(urls));
        self
    }

    pub fn video_url(mut self, url: String) -> Self {
        self.0.push(Media::VideoUrl(url));
        self
    }

    pub fn build(self, role: Role) -> Message {
        Message::Media {
            role,
            content: self.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_der_message() {
        let json = r#"{"role":"user","content":[{"type":"image_url","image_url":{"url":"https://www.baidu.com/img/bd_logo.png"}},{"type":"text","text":"这是什么"}]}"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        if let Message::Media { content, .. } = message {
            assert_eq!(
                content[0],
                Media::ImageUrl("https://www.baidu.com/img/bd_logo.png".to_string())
            );
            assert_eq!(content[1], Media::Text("这是什么".to_string()));
        } else {
            panic!("message is not 'Message::Media'");
        }
    }
}
