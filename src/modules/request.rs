use std::sync::Arc;
use std::error::Error;
use tokio::sync::Semaphore;
use reqwest::Client;


pub async fn make_request(url: String, client: Client, sem: Arc<Semaphore>) -> Result<String, Box<dyn Error>> {
    let _permit = sem.acquire().await?;
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}
