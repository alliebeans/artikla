use reqwest::{Url, Error};

pub async fn get_body(link: String) -> Result<String, Error> {
    let url = Url::parse(&link).expect("Url bad format");
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    return Ok(body);
}