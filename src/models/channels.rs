/* use std::fs::File;
use std::io::BufReader; */
use std::error::Error;
use rss::{Channel, Item};
use super::article::Article;

/* pub async fn get_news() -> Vec<Article> {
    let mut articles = Vec::new();
    let channel_str = get_all_channel_str();
    
    for url in channel_str {
        let feed = get_feed(url).await.unwrap();
        let feed_articles = create_new_articles(feed.items);
        for a in feed_articles {
            articles.push(a);
        }
    }
    
    return articles;
} */

/* fn get_all_channel_str<'a>() -> Vec<&'a str> {
    const AFTONBLADET: &str = "https://rss.aftonbladet.se/rss2/small/pages/sections/senastenytt/";
    const SVT: &str = "http://www.svt.se/nyheter/rss.xml";
    const DN: &str = "http://www.dn.se/nyheter/m/rss/";
    const SVD: &str = "https://www.svd.se/feed/articles.rss";

    let channels = vec![AFTONBLADET, SVT, DN, SVD];
    return channels;
} */

/* pub fn get_feed_from_file() -> Vec<Article> {
    let file = File::open("/home/alliebeans/Documents/1_studier/0_htmx/news_reader_app/rss/svt/31aug.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    let articles = create_new_articles(channel.items);
    return articles;
} */

pub async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn create_new_articles(items: Vec<Item>) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();
    for item in items {
        let title = item.title.unwrap();
        let link = item.link.unwrap();
        let published = item.pub_date.unwrap();
        let article = Article::create(&title, &link, &published);
        articles.push(article);
    }
    return articles;
}