/* use std::env;
use dotenvy::dotenv;
use sqlx::Pool;
use sqlx::postgres::Postgres;

use models::article::Article;
use models::channels::{get_feed, create_new_articles};

pub mod models;

pub async fn establish_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&db_url).await?;

    return Ok(pool)
}


pub async fn init_channel_test() {
    let feed = get_feed("https://www.svd.se/feed/articles/category/sverige.rss")
    .await
    .unwrap();

    let articles = create_new_articles(feed.items);
    /* insert_articles(pool, articles); */
}

/// *********************************************
///                 A R T I C L E S
/// *********************************************
/// 

pub fn insert_articles(pool: &Pool<Postgres>, articles: Vec<Article>) -> () {
    for a in articles {
        let _record = sqlx::query("insert into articles (id, title, link, published, topic) values '$1', '$2', '$3', '$4', $5;")
            .bind(a.id)
            .bind(a.title)
            .bind(a.link)
            .bind(a.published)
            .bind(a.topic)
            .execute(pool);
    }
}

pub async fn get_article(pool: &Pool<Postgres>, uuid: uuid::Uuid) -> Article {
    let results = sqlx::query_as::<_, Article>("select * from articles where id = $1 limit 1")
        .bind(uuid)
        .fetch_one(pool)
        .await
        .unwrap();

    return results;
}

pub async fn get_articles(pool: &Pool<Postgres>, limit: i32) -> Vec<Article> {
    let results = sqlx::query_as::<_, Article>("select * from articles order by published desc limit $1")
        .bind(limit)
        .fetch_all(pool)
        .await
        .unwrap();

    return results;
}

pub async fn get_articles_paginator(pool: &Pool<Postgres>, uuid: uuid::Uuid, limit: i32) -> Vec<Article> {
    let results = sqlx::query_as::<_, Article>("select * from articles where published < (select published from articles where ('$1' ~ E'^[[:xdigit:]]{{8}}-([[:xdigit:]]{{4}}-){{3}}[[:xdigit:]]{{12}}$' and id = '$1')) order by published desc limit $2;")
        .bind(uuid)
        .bind(limit)
        .fetch_all(pool)
        .await
        .unwrap();

    return results;
} */