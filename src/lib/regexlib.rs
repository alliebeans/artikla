use regex::Regex;

use crate::models::publication::Publication;

pub fn get_regex_url() -> Regex {
    return Regex::new(r"^(?:https?:\/\/)?(?:[^@\/\n]+@)?(?:www\.)?(\w+)").unwrap();
}

/* pub fn get_regex_topic(publication: Publication) -> Regex {
    match publication {
        Publication::Dn => ,
        _ => "Not implemented",
    }
} */