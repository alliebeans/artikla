use std::error::Error;
use rss::{Channel, Item};
use super::article::Article;
use super::publication::Publication;

pub async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn create_new_articles(items: Vec<Item>, publication: Publication) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();
    for item in items {
        let title = item.title.unwrap();
        let link = item.link.unwrap();
        let published = item.pub_date.unwrap();
        let article = Article::create(&title, &link, &published, publication);
        articles.push(article);
    }
    return articles;
}
