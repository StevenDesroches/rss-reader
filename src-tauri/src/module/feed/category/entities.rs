use crate::module::feed::entities::Feed;

pub struct Category {
    pub id: i32,
    pub title: String,
    pub parent_id: Option<i32>,
    pub feeds: Option<Vec<Feed>>,
}

pub struct CategoryBuilder {
    id: Option<i32>,
    title: Option<String>,
    parent_id: Option<i32>,
    feeds: Option<Vec<Feed>>,
}

impl Category {
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder {
            id: None,
            title: None,
            parent_id: None,
            feeds: None,
        }
    }
}

impl CategoryBuilder {
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn parent(mut self, id: i32) -> Self {
        self.parent_id = Some(id);
        self
    }

    pub fn feeds(mut self, feeds: Vec<Feed>) -> Self {
        self.feeds = Some(feeds);
        self
    }

    pub fn build(self) -> Category {
        Category {
            id: self.id.unwrap_or(0),
            title: self.title.unwrap_or("".to_string()),
            parent_id: self.parent_id,
            feeds: self.feeds,
        }
    }
}
