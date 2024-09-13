use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::service::http::IHttp;
use crate::shared::types::Url;

#[derive(Debug, PartialEq)]
enum FeedType {
    Rss,
    Atom,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedArticle {
    pub(super) id: Option<i32>,
    pub(super) title: String,
    pub(super) link: Option<String>,
    pub(super) content: String,
    pub(super) pub_date: Option<String>,
}

impl FeedArticle {
    pub(super) fn from_model(id: i32, title: String, content: String) -> Self {
        Self {
            id: Some(id),
            title,
            link: None,
            content,
            pub_date: None,
        }
    }

    pub(super) fn from_rss(item: Item) -> Self {
        Self {
            id: None,
            title: item.title,
            link: None, // rss link handling
            content: item.description,
            pub_date: item.pub_date,
        }
    }

    pub(super) fn from_atom(entry: AtomEntry) -> Self {
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
    pub(super) id: Option<i32>,
    pub(super) title: String,
    pub(super) xml_url: Url,
    pub(super) link: Option<Url>,
    pub(super) description: Option<String>,
    pub(super) articles: Vec<FeedArticle>,
    pub(super) category_id: Option<i32>,
}

pub(super) struct FeedBuilder {
    id: Option<i32>,
    title: Option<String>,
    xml_url: Option<Url>,
    link: Option<Url>,
    description: Option<String>,
    articles: Option<Vec<FeedArticle>>,
    category_id: Option<i32>,
}

impl FeedBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn xml_url(mut self, xml_url: Url) -> Self {
        self.xml_url = Some(xml_url);
        self
    }

    pub fn _link(mut self, link: Url) -> Self {
        self.link = Some(link);
        self
    }

    pub fn _description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn _articles(mut self, articles: Vec<FeedArticle>) -> Self {
        self.articles = Some(articles);
        self
    }

    pub fn category_id(mut self, category_id: i32) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn build(self) -> Feed {
        Feed {
            id: self.id,
            title: self.title.unwrap_or("".to_string()),
            xml_url: self.xml_url.unwrap_or("".to_string()),
            link: self.link,
            description: self.description,
            articles: self.articles.unwrap_or(Vec::new()),
            category_id: self.category_id,
        }
    }
}

impl Feed {
    pub(super) fn builder() -> FeedBuilder {
        FeedBuilder {
            id: None,
            title: None,
            xml_url: None,
            link: None,
            description: None,
            articles: None,
            category_id: None,
        }
    }

    pub(super) fn from_rss(rss: RssFeed) -> Self {
        Self {
            id: None,
            title: rss.channel.title,
            xml_url: String::new(),
            link: None,        // Rss link handling
            description: None, //Rss.channel.description,
            articles: rss
                .channel
                .items
                .into_iter()
                .map(FeedArticle::from_rss)
                .collect(),
            category_id: None,
        }
    }

    pub(super) fn from_atom(atom: AtomFeed) -> Self {
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
            category_id: None,
        }
    }

    pub(super) async fn from_url(url: Url) -> Result<Self> {
        let http = crate::service::http::HttpReqwest {};

        let content = http
            .fetch(&url)
            .await
            .map_err(|e| Error::Entity(e.to_string()))?;

        let reader = quick_xml::Reader::from_str(&content);

        let mut feed: Self = match Self::determine_feed_type(reader) {
            FeedType::Unknown => Err(Error::XmlBadFormat)?,
            FeedType::Rss => {
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
                        b"rss" => FeedType::Rss,
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
pub(super) struct AtomFeed {
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
pub(super) struct AtomEntry {
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
pub(super) struct AtomLink {
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
pub(super) struct AtomPerson {
    pub name: Option<String>,
    pub email: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct AtomContent {
    #[serde(rename = "@type", default)]
    pub content_type: Option<String>,
    #[serde(rename = "$value")]
    pub content: String,
}

// use chrono::{DateTime, FixedOffset};
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RssFeed {
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Channel {
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
pub(super) struct Item {
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
pub(super) struct Category {
    pub domain: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Enclosure {
    pub url: Option<String>,
    pub length: Option<i32>,
    #[serde(rename = "type")]
    pub enclosure_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Guid {
    #[serde(rename = "@isPermaLink", default)]
    pub is_permalink: Option<bool>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Source {
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}
