use sqlx::PgPool;
use rocket_dyn_templates::{Template, context};
use crate::models::{topic::Topic, article::Article, url::get_body};

#[get("/articles")]
pub async fn articles(pool: &rocket::State<PgPool>) -> Template {
    let limit: i64 = 15;
    let topics = Topic::get_all_as_string();
    let articles = sqlx::query_as::<_, Article>("select * from articles order by published desc limit $1")
        .bind(limit)
        .fetch_all(pool.inner())
        .await
        .unwrap();

    Template::render("articles", context! { topics: topics, articles: articles, limit: limit })
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
    let topics = Topic::get_all_as_string();
    let limit = 11;
    let uuid: uuid::Uuid = uuid::Uuid::parse_str(&uuid_str).unwrap();

    let query = format!("select * from articles where published < (select published from articles where ('{uuid}' ~ E'^[[:xdigit:]]{{8}}-([[:xdigit:]]{{4}}-){{3}}[[:xdigit:]]{{12}}$' and id = '{uuid}')) order by published desc limit {limit}");

    let articles = sqlx::query_as::<_, Article>(&query)
    .fetch_all(pool.inner())
    .await
    .unwrap();

    Template::render("articles_list", context! { topics: topics, articles: articles, limit: limit})
}