use reqwest::Error;

pub async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}

