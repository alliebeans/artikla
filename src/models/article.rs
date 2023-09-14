use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::models::topic::Topic;

use super::publication::Publication;

#[derive(
    Serialize, Clone, Debug, PartialEq, FromRow
)]
#[sqlx(rename_all = "lowercase")]

pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub link: String,
    pub published: DateTime<Utc>,
    pub topic: Topic,
    pub publication: Publication,
}
impl Article {
    pub fn create(title: &str, link: &str, published: &str, publication: Publication) -> Article {
        let id = Uuid::new_v4();
        let topic: Topic = Topic::Ekonomi;
        return Article { id: id, title: String::from(title), link: String::from(link), published: DateTime::parse_from_rfc2822(published).unwrap().into(), topic: topic, publication: publication }
    }
}