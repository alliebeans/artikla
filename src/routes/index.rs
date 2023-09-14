use std::str::FromStr;

use sqlx::PgPool;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use crate::models::{channels, publication::Publication};
use regex::Regex;

#[get("/")]
pub fn index() -> Template {
    Template::render("base", context! { value: "" })
}

#[get("/login")]
pub fn login() -> Redirect {
    return Redirect::to("/articles");
}

#[get("/update")]
pub async fn update(pool: &rocket::State<PgPool>) -> () {
    let url = "https://www.svt.se/nyheter/granskning/rss.xml";
    let feed = channels::get_feed(url)
    .await
    .unwrap();

    let re = Regex::new(r"^(?:https?:\/\/)?(?:[^@\/\n]+@)?(?:www\.)?(\w+)").unwrap();
    let publication_string: String = re.captures(url).unwrap()[0].into();
    let publication = Publication::from_str(&publication_string).unwrap();

    let articles = channels::create_new_articles(feed.items, publication);
    
    for a in articles {
        let _record = sqlx::query("insert into articles (id, title, link, published, topic) values ($1, $2, $3, $4, $5)")
            .bind(a.id)
            .bind(a.title)
            .bind(a.link)
            .bind(a.published)
            .bind(a.topic)
            .execute(pool.inner())
            .await;
    }
}