use std::sync::Arc;
use std::error::Error;
use tokio::sync::Semaphore;
use reqwest::Client;


#[derive(Clone)]
pub struct Request {
    pub url: String,
    client: Client,
    semaphore: Arc<Semaphore>
}

impl Request {
    pub fn new(url: String, client: Client, semaphore: Arc<Semaphore>) -> Request {
        Request {
            url,
            client,
            semaphore
        }
    }
    pub async fn send(self) -> Result<String, Box<dyn Error>> {
        let _permit = self.semaphore.acquire().await?;
        let response = self.client.get(self.url).send().await?;
        let status = response.status().to_string();
        Ok(status)
    }
}
