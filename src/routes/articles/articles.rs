use std::str::FromStr;
use sqlx::PgPool;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use chrono::{DateTime, Utc};
use crate::models::{topic::Topic, article::Article, url::get_body};
use crate::lib::date_fmt_swedish;
use crate::lib::regexlib;

use crate::models::{channels, publication::Publication};


#[get("/articles")]
pub async fn articles(pool: &rocket::State<PgPool>) -> Template {
    let limit: i64 = get_limit();
    let topics = Topic::get_all_as_string();
    let articles = sqlx::query_as::<_, Article>("select * from articles order by published desc limit $1")
        .bind(limit)
        .fetch_all(pool.inner())
        .await
        .unwrap();
    let today = date_fmt_swedish::get_today_swedish(chrono::offset::Local::now());
    let weather: String = "11°".to_string();

    Template::render("articles", context! { topics: topics, articles: articles, limit: limit, today: today, weather: weather })
}

#[get("/articles/id/<uuid_str>")]
pub async fn expand_article(pool: &rocket::State<PgPool>, uuid_str: String) -> Template {
    let uuid = uuid::Uuid::parse_str(&uuid_str).unwrap();
    let article = sqlx::query_as::<_, Article>("select * from articles where id = $1 limit 1")
    .bind(uuid)
    .fetch_one(pool.inner())
    .await
    .unwrap();

    Template::render("expand_article", context! { article: article})
}

#[get("/articles/<uuid_str>/content/1")]
pub async fn full_article_content(pool: &rocket::State<PgPool>, uuid_str: String) -> String {
    let uuid = uuid::Uuid::parse_str(&uuid_str).unwrap();
    let article = sqlx::query_as::<_, Article>("select * from articles where id = $1 limit 1")
    .bind(uuid)
    .fetch_one(pool.inner())
    .await
    .unwrap();
    let link = String::from(&article.link);
    let body = get_body(link).await.unwrap();

    return body;
}

#[get("/articles/<uuid_str>")]
pub async fn articles_paginator(pool: &rocket::State<PgPool>, uuid_str: String) -> Template {
    let limit = get_limit();
    let uuid: uuid::Uuid = uuid::Uuid::parse_str(&uuid_str).unwrap();

    let query = format!("select * from articles where published < (select published from articles where ('{uuid}' ~ E'^[[:xdigit:]]{{8}}-([[:xdigit:]]{{4}}-){{3}}[[:xdigit:]]{{12}}$' and id = '{uuid}')) order by published desc limit {limit}");

    let articles = sqlx::query_as::<_, Article>(&query)
    .fetch_all(pool.inner())
    .await
    .unwrap();

    Template::render("articles_list", context! { articles: articles, limit: limit})
}

#[post("/sync")]
pub async fn sync_articles(pool: &rocket::State<PgPool>) -> Template {
    let limit = get_limit();
    let mut new_articles = Vec::new();
    
    let url = "https://www.svt.se/nyheter/vetenskap/rss.xml";
    let publication_string: String = regexlib::get_regex_url().captures(url).unwrap()[1].into();
    let publication: Publication = Publication::from_str(&publication_string).unwrap();

    let latest = get_latest_datetime(pool, publication).await;

    let feed = channels::get_feed(url)
    .await
    .unwrap();

    for item in feed.items {
        let datetime: DateTime<Utc> = DateTime::parse_from_rfc2822(item.pub_date.as_ref().unwrap().as_str()).unwrap().into();
        if datetime <= latest {
            break;
        }
        let article = channels::create_new_article(item.title.unwrap(), item.link.unwrap(), item.pub_date.unwrap(), publication);
        new_articles.push(article);
    }
    
    for a in new_articles {
        let _record = sqlx::query("insert into articles (id, title, link, published, topic, publication) values ($1, $2, $3, $4, $5, $6)")
            .bind(a.id)
            .bind(a.title)
            .bind(a.link)
            .bind(a.published)
            .bind(a.topic)
            .bind(a.publication)
            .execute(pool.inner())
            .await;
    }

    let articles = sqlx::query_as::<_, Article>("select * from articles order by published desc limit $1")
        .bind(limit)
        .fetch_all(pool.inner())
        .await
        .unwrap();

    return Template::render("articles_list", context! { articles: articles, limit: limit })
}

#[post("/init")]
pub async fn init_articles(pool: &rocket::State<PgPool>) -> Template {
    let limit = get_limit();
    let url = "https://www.svt.se/nyheter/vetenskap/rss.xml";
    let publication_string: String = regexlib::get_regex_url().captures(url).unwrap()[1].into();
    let publication: Publication = Publication::from_str(&publication_string).unwrap();

    let feed = channels::get_feed(url)
    .await
    .unwrap();

    let new_articles = channels::create_new_articles(feed.items, publication);
    
    for a in new_articles {
        let _record = sqlx::query("insert into articles (id, title, link, published, topic, publication) values ($1, $2, $3, $4, $5, $6)")
            .bind(a.id)
            .bind(a.title)
            .bind(a.link)
            .bind(a.published)
            .bind(a.topic)
            .bind(a.publication)
            .execute(pool.inner())
            .await;
    }

    let articles = sqlx::query_as::<_, Article>("select * from articles order by published desc limit $1")
    .bind(limit)
    .fetch_all(pool.inner())
    .await
    .unwrap();

    return Template::render("articles_list", context! { articles: articles, limit: limit })
}


async fn get_latest_datetime(pool: &rocket::State<PgPool>, publication: Publication) -> DateTime<Utc> {
    let limit = 1;
    let latest: DateTime<Utc> = sqlx::query_as::<_, Article>("select * from articles where publication = $1 order by published desc limit $2")
    .bind(publication)
    .bind(limit)
    .fetch_all(pool.inner())
    .await
    .unwrap()[0].published;
    
    return latest;
}

async fn articles_is_empty(pool: &rocket::State<PgPool>) -> bool {
    let count: (i64,) = sqlx::query_as("select count(title) from articles")
        .fetch_one(pool.inner()).await.unwrap();

    if count.0 == 0 {
        return true
    }
    return false
}

fn get_limit() -> i64 {
    return 11
}