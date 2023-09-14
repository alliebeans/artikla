use std::str::FromStr;
use std::fmt::Display;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")] 
pub enum Topic {
    Inrikes,
    Utrikes,
    Lokalt,
    Ekonomi,
    Kultur,
    Vetenskap,
    Sport,
    Opinion,
}
impl Topic {
    pub fn get_all_as_string() -> Vec<String> {
        let topics = vec![
        String::from(Topic::Inrikes),
        String::from(Topic::Utrikes),
        String::from(Topic::Lokalt),
        String::from(Topic::Ekonomi),
        String::from(Topic::Kultur),
        String::from(Topic::Vetenskap),
        String::from(Topic::Sport),
        String::from(Topic::Opinion),
        ];
        return topics;
    }
}
impl FromStr for Topic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inrikes" => Ok(Topic::Inrikes),
            "utrikes" => Ok(Topic::Utrikes),
            "lokalt" => Ok(Topic::Lokalt),
            "ekonomi" => Ok(Topic::Ekonomi),
            "kultur" => Ok(Topic::Kultur),
            "vetenskap" => Ok(Topic::Vetenskap),
            "sport" => Ok(Topic::Sport),
            "opinion" => Ok(Topic::Opinion),
            _ => Err("Topic not implemented"),
        }
    }
}
impl From<Topic> for String {
    fn from(topic: Topic) -> String {
        match topic {
            Topic::Inrikes => String::from("Inrikes"),
            Topic::Utrikes => String::from("Utrikes"),
            Topic::Lokalt => String::from("Lokalt"),
            Topic::Ekonomi => String::from("Ekonomi"),
            Topic::Kultur => String::from("Kultur"),
            Topic::Vetenskap => String::from("Vetenskap"),
            Topic::Sport => String::from("Sport"),
            Topic::Opinion => String::from("Opinion"),
        }    
    }
}
impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}