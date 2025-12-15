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

#[derive(PartialEq, Clone, Debug)]
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
    fn new<T: AsRef<str>>(value: T) -> Self {
        Self {
            value: value.as_ref().to_owned(),
        }
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
                map.serialize_entry("image_url", &Url::new(value))?;
            }
            Media::Video(value) => {
                map.serialize_entry("type", "video")?;
                map.serialize_entry("video", value)?;
            }
            Media::VideoUrl(value) => {
                map.serialize_entry("type", "video_url")?;
                map.serialize_entry("video_url", &Url::new(value))?;
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
    Text(TextMessage),
    Media(MediaMessage),
}

impl Message {
    pub fn text<T: AsRef<str>>(role: Role, content: T) -> Self {
        Message::Text(TextMessage::new(role, content))
    }

    pub fn media(role: Role) -> MediaMessage {
        MediaMessage::new(role)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextMessage {
    role: Role,
    content: String,
}

impl TextMessage {
    pub fn new<T: AsRef<str>>(role: Role, content: T) -> Self {
        TextMessage {
            role,
            content: content.as_ref().to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaMessage {
    role: Role,
    content: Vec<Media>,
}

impl MediaMessage {
    pub fn new(role: Role) -> Self {
        MediaMessage {
            role,
            content: Vec::new(),
        }
    }

    pub fn content(mut self, content: Vec<Media>) -> Self {
        self.content = content;
        self
    }

    pub fn text<T: AsRef<str>>(mut self, content: T) -> Self {
        self.content.push(Media::Text(content.as_ref().to_owned()));
        self
    }

    pub fn image_url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.content.push(Media::ImageUrl(url.as_ref().to_owned()));
        self
    }

    pub fn video<T: AsRef<str>>(mut self, urls: Vec<T>) -> Self {
        self.content.push(Media::Video(
            urls.into_iter()
                .map(|url| url.as_ref().to_owned())
                .collect(),
        ));
        self
    }

    pub fn video_url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.content.push(Media::VideoUrl(url.as_ref().to_owned()));
        self
    }
}

impl From<MediaMessage> for Message {
    fn from(message: MediaMessage) -> Self {
        Message::Media(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_der_message() {
        let json = r#"{"role":"user","content":[{"type":"image_url","image_url":{"url":"https://www.baidu.com/img/bd_logo.png"}},{"type":"text","text":"这是什么"}]}"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        if let Message::Media(MediaMessage { content, .. }) = message {
            if let Media::ImageUrl(url) = &content[0] {
                assert_eq!(url, "https://www.baidu.com/img/bd_logo.png");
            } else {
                panic!("'content[0]' is not 'Media::ImageUrl'");
            }
            if let Media::Text(text) = &content[1] {
                assert_eq!(text, "这是什么");
            } else {
                panic!("'content[1]' is not 'Media::Text'");
            }
        } else {
            panic!("'message' is not 'Message::Media'");
        }
    }
}
