use serde::{Deserialize, Serialize};

// use crate::shared::errors::*;
use super::super::entities;
use entities::AtomEntry;
use entities::Item;

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub(super) id: Option<i32>,
    pub(super) feed_id: Option<Vec<i32>>,
    pub(super) title: String,
    pub(super) link: Option<String>,
    pub(super) content: String,
    pub(super) pub_date: Option<String>,
}

impl Article {
    // pub(super) fn from_model(id: i32, title: String, content: String) -> Self {
    //     Self {
    //         id: Some(id),
    //         title,
    //         link: None,
    //         content,
    //         pub_date: None,
    //     }
    // }

    pub(super) fn builder() -> ArticleBuilder {
        ArticleBuilder {
            id: None,
            feed_id: None,
            title: None,
            link: None,
            content: None,
            pub_date: None,
        }
    }

    #[allow(private_interfaces)]//TODO: revisit later
    pub fn from_rss(item: Item) -> Self {
        Self {
            id: None,
            feed_id: None,
            title: item.title,
            link: None, // rss link handling
            content: item.description,
            pub_date: item.pub_date,
        }
    }
    
    #[allow(private_interfaces)]//TODO: revisit later
    pub fn from_atom(entry: AtomEntry) -> Self {
        Self {
            id: None,
            feed_id: None,
            title: entry.title,
            // link: entry.link.map(|l| l.href),
            link: None,
            content: entry.content.content,
            pub_date: entry.published,
        }
    }
}

pub(super) struct ArticleBuilder {
    id: Option<i32>,
    feed_id: Option<Vec<i32>>,
    title: Option<String>,
    link: Option<String>,
    content: Option<String>,
    pub_date: Option<String>,
}

impl ArticleBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }
    pub fn feed_id(mut self, feed_id: Vec<i32>) -> Self {
        self.feed_id = Some(feed_id);
        self
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    pub fn _link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    pub fn _pub_date(mut self, pub_date: String) -> Self {
        self.pub_date = Some(pub_date);
        self
    }

    pub fn build(self) -> Article {
        Article {
            id: self.id,
            feed_id: self.feed_id,
            title: self.title.unwrap_or("".to_string()),
            link: self.link,
            content: self.content.unwrap_or("".to_string()),
            pub_date: self.pub_date,
        }
    }
}
