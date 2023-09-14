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
        let article = create_new_article(item.title.unwrap(), item.link.unwrap(), item.pub_date.unwrap(), publication);
        articles.push(article);
    }
    return articles;
}

pub fn create_new_article(title: String, link: String, published: String, publication: Publication) -> Article {
    let article = Article::create(&title, &link, &published, publication);
    return article;
}