use std::str::FromStr;
use std::fmt::Display;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")] 
pub enum Publication {
    Dn,
    Svt,
    Aftonbladet,
    Svd
}
impl Publication {
    pub fn get_all_as_string() -> Vec<String> {
        let publications = vec![
        String::from(Publication::Dn),
        String::from(Publication::Svt),
        String::from(Publication::Aftonbladet),
        String::from(Publication::Svd),
        ];
        return publications;
    }
}
impl FromStr for Publication {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dn" => Ok(Publication::Dn),
            "svt" => Ok(Publication::Svt),
            "aftonbladet" => Ok(Publication::Aftonbladet),
            "svd" => Ok(Publication::Svd),
            _ => Err("Publication not implemented"),
        }
    }
}
impl From<Publication> for String {
    fn from(topic: Publication) -> String {
        match topic {
            Publication::Dn => String::from("Dagens Nyheter"),
            Publication::Svt => String::from("SVT Nyheter"),
            Publication::Aftonbladet => String::from("Aftonbladet"),
            Publication::Svd => String::from("Svenska Dagbladet"),
        }
    }
}
impl From<Publication> for &'static str {
    fn from(topic: Publication) -> &'static str {
        match topic {
            Publication::Dn => "Dagens Nyheter",
            Publication::Svt => "SVT Nyheter",
            Publication::Aftonbladet => "Aftonbladet",
            Publication::Svd => "Svenska Dagbladet",
        }
    }
}
impl Display for Publication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}