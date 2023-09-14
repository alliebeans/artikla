use serde::{Serialize, Serializer, ser::SerializeStruct};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use sqlx::FromRow;
use crate::models::topic::Topic;
use crate::lib::date_fmt_swedish;

use super::publication::Publication;

#[derive(
    Clone, Debug, PartialEq, FromRow
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
    pub fn get_published_format_swedish(datetime: DateTime<Utc>) -> String {
        let time = datetime.format("%R").to_string();

        if datetime == Utc::now() {
            return format!("Idag, {}", time.as_str());
        } else if datetime == Utc::now() - Duration::days(1) {
            return format!("Igår, {}", time.as_str());
        } else {
            let day = format!("{} ", datetime.format("%-e"));
            let month_name = date_fmt_swedish::get_month_name_swedish(datetime);
            let year = format!("{} ", datetime.format("%Y"));
            return format!("{} {}, {}", day, month_name, year);
        }
    }
}
impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("link", &self.link)?;
        state.serialize_field("published", &Article::get_published_format_swedish(self.published))?;
        state.serialize_field("topic", &self.topic)?;
        state.serialize_field("publication", &self.publication)?;
        state.end()
    }
}