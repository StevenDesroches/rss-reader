use serde::{Deserialize, Serialize};

use crate::service::http::IHttp;
use crate::error::{Error, Result};
use crate::shared::types::Url;

#[derive(Debug, PartialEq)]
enum FeedType {
    RSS,
    Atom,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedArticle {
    pub id: Option<i32>,
    pub title: String,
    pub link: Option<String>,
    pub content: String,
    pub pub_date: Option<String>,
}

impl FeedArticle {
    pub fn from_model(id: i32, title: String, content: String) -> Self {
        Self {
            id: Some(id),
            title: title,
            link: None,
            content: content,
            pub_date: None,
        }
    }

    pub fn from_rss(item: Item) -> Self {
        Self {
            id: None,
            title: item.title,
            link: None, // RSS link handling
            content: item.description,
            pub_date: item.pub_date,
        }
    }

    pub fn from_atom(entry: AtomEntry) -> Self {
        Self {
            id: None,
            title: entry.title,
            // link: entry.link.map(|l| l.href),
            link: None,
            content: entry.content.content,
            pub_date: entry.published,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feed {
    pub id: Option<i32>,
    pub title: String,
    pub xml_url: Url,
    pub link: Option<Url>,
    pub description: Option<String>,
    pub articles: Vec<FeedArticle>,
}

impl Feed {
    pub fn from_model(id: i32, title: String, xml_url: Url) -> Self {
        Self {
            id: Some(id),
            title: title,
            xml_url: xml_url,
            link: None,
            description: None,
            articles: Vec::new(),
        }
    }

    pub fn from_rss(rss: RssFeed) -> Self {
        Self {
            id: None,
            title: rss.channel.title,
            xml_url: String::new(),
            link: None,        // RSS link handling
            description: None, //rss.channel.description,
            articles: rss
                .channel
                .items
                .into_iter()
                .map(FeedArticle::from_rss)
                .collect(),
        }
    }

    pub fn from_atom(atom: AtomFeed) -> Self {
        Self {
            id: None,
            title: atom.title,
            xml_url: String::new(),
            // link: atom.link.map(|l| l.href),
            link: None,
            description: None, // Atom description handling
            articles: atom
                .entries
                .into_iter()
                .map(FeedArticle::from_atom)
                .collect(),
        }
    }

    pub async fn from_url(url: Url) -> Result<Self> {
        let http = crate::service::http::HttpReqwest {};

        let content = http
            .fetch(&url)
            .await
            .map_err(|e| Error::Entity(e.to_string()))?;

        let reader = quick_xml::Reader::from_str(&content);

        let mut feed: Self = match Self::determine_feed_type(reader) {
            FeedType::Unknown => Err(Error::XmlBadFormat)?,
            FeedType::RSS => {
                let rss: RssFeed = quick_xml::de::from_str(&content)
                    .map_err(|e| Error::XmlDeserialize(e.to_string()))?;
                Feed::from_rss(rss)
            }
            FeedType::Atom => {
                let atom: AtomFeed = quick_xml::de::from_str(&content)
                    .map_err(|e| Error::XmlDeserialize(e.to_string()))?;
                Feed::from_atom(atom)
            }
        };

        feed.xml_url = url.to_owned();

        Ok(feed)
    }

    fn determine_feed_type(mut reader: quick_xml::Reader<&[u8]>) -> FeedType {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    return match e.name().as_ref() {
                        b"rss" => FeedType::RSS,
                        b"feed" => FeedType::Atom,
                        _ => FeedType::Unknown,
                    };
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                    return FeedType::Unknown;
                }
                _ => (),
            }
            buf.clear();
        }
        FeedType::Unknown
    }
}










#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomFeed {
    pub title: String,
    pub subtitle: Option<String>,
    // pub links: Vec<AtomLink>,
    // pub id: String,
    // pub updated: String,
    // // pub updated: DateTime<FixedOffset>,
    // pub authors: Vec<AtomPerson>,
    #[serde(rename = "entry", default)]
    pub entries: Vec<AtomEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomEntry {
    pub title: String,
    #[serde(rename = "link", default)]
    pub links: Option<Vec<AtomLink>>,
    pub id: Option<String>,
    pub updated: Option<String>,
    pub published: Option<String>,
    // pub updated: DateTime<FixedOffset>,
    // pub published: Option<DateTime<FixedOffset>>,
    #[serde(rename = "author", default)]
    pub authors: Option<Vec<AtomPerson>>,
    pub content: AtomContent,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtomLink {
    #[serde(rename = "@href", default)]
    pub href: Option<String>,
    #[serde(rename = "@rel", default)]
    pub rel: Option<String>,
    #[serde(rename = "@type")]
    pub link_type: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtomPerson {
    pub name: Option<String>,
    pub email: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtomContent {
    #[serde(rename = "@type", default)]
    pub content_type: Option<String>,
    #[serde(rename = "$value")]
    pub content: String,
}









// use chrono::{DateTime, FixedOffset};
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RssFeed {
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub title: String,
    // pub link: String,
    pub description: Option<String>,
    // pub language: Option<String>,
    // pub copyright: Option<String>,
    // #[serde(rename = "managingEditor")]
    // pub managing_editor: Option<String>,
    // pub webmaster: Option<String>,
    // #[serde(rename = "pubDate")]
    // pub pub_date: Option<String>,
    // // pub pub_date: Option<DateTime<FixedOffset>>,
    // #[serde(rename = "lastBuildDate")]
    // pub last_build_date: Option<String>,
    // // pub last_build_date: Option<DateTime<FixedOffset>>,
    // pub category: Option<String>,
    // pub generator: Option<String>,
    // pub docs: Option<String>,
    // pub cloud: Option<Cloud>,
    // pub ttl: Option<i32>,
    // pub image: Option<Image>,
    // pub rating: Option<String>,
    // #[serde(rename = "textInput")]
    // pub text_input: Option<TextInput>,
    // #[serde(rename = "skipHours")]
    // pub skip_hours: Option<Vec<i32>>,
    // #[serde(rename = "skipDays")]
    // pub skip_days: Option<Vec<String>>,
    #[serde(rename = "item", default)]
    pub items: Vec<Item>,
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Cloud {
//     pub domain: String,
//     pub port: i32,
//     pub path: String,
//     pub register_procedure: String,
//     pub protocol: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Image {
//     pub url: String,
//     pub title: String,
//     pub link: String,
//     pub width: Option<i32>,
//     pub height: Option<i32>,
//     pub description: Option<String>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TextInput {
//     pub title: String,
//     pub description: String,
//     pub name: String,
//     pub link: String,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub title: String,
    pub link: Option<String>,
    pub description: String,
    pub author: Option<String>,
    pub category: Option<Vec<Category>>,
    pub comments: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<Guid>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    // pub pub_date: Option<DateTime<FixedOffset>>,
    pub source: Option<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub domain: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Enclosure {
    pub url: Option<String>,
    pub length: Option<i32>,
    #[serde(rename = "type")]
    pub enclosure_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid {
    #[serde(rename = "@isPermaLink", default)]
    pub is_permalink: Option<bool>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}
