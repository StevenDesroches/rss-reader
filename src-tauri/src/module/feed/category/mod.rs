
pub struct FeedCategoryController {}

impl FeedCategoryController {
    pub fn new() -> Self {
        FeedCategoryController {}
    }

    // pub fn get_all(&self) -> Result<Vec<Feed>> {
    //     let db = Db::new().open();
    //     let mut feeds = Vec::new();
    //     let db_feeds = db.get_feeds()?;
    //     db.close()?;

    //     let _: Vec<_> = db_feeds
    //         .into_iter()
    //         .map(|i| feeds.push(Feed::from_model(i.0, i.1, i.2)))
    //         .collect();

    //     Ok(feeds)
    // }

    // pub fn get_by_feed($self, feed_id: i32){
    //     let db = Db::new().open();
    //     let mut feeds = Vec::new();
    //     let db_feeds = db.get_feeds()?;
    //     db.close()?;

    //     let _: Vec<_> = db_feeds
    //         .into_iter()
    //         .map(|i| feeds.push(Feed::from_model(i.0, i.1, i.2)))
    //         .collect();

    //     Ok(feeds)
    // }

    //Get a feed from the db
    // pub fn get(url: Url, title: String) -> Feed {

    // }

    // pub fn save() {}
}
