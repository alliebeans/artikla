use sqlx::PgPool;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use crate::models::channels::{get_feed, create_new_articles};

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
    let feed = get_feed("https://www.svd.se/feed/articles/category/sverige.rss")
    .await
    .unwrap();

    let articles = create_new_articles(feed.items);
    
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