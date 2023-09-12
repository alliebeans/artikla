#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;

use routes::index::{index, login, update};
use routes::articles::articles::{articles, articles_paginator, expand_article, full_article_content};

mod models;
mod routes;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build().manage::<PgPool>(pool).attach(Template::fairing()).mount("/public", FileServer::from("static")).mount("/", routes![index, login, update, articles, articles_paginator, expand_article, full_article_content])
}