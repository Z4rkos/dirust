use std::sync::Arc;
use std::error::Error;
use tokio::sync::Semaphore;
use reqwest::Client;


#[derive(Clone)]
pub struct Request {
    pub url: String,
    client: Client,
    pub semaphore: Arc<Semaphore>
}
impl Request {
    pub fn new(url: String, client: Client, semaphore: Arc<Semaphore>) -> Request {
        Request {
            url,
            client,
            semaphore
        }
    }
    pub async fn send(self, word: String) -> Result<String, Box<dyn Error>> {
        let _permit = self.semaphore.acquire().await?;
        let url = format!("{}/{}", self.url, word);
        let response = self.client.get(url).send().await?;
        let status = response.status().to_string();
        Ok(status)
    }
}

pub struct RequestBuilder {
    url: String,
    client: Client,
    semaphore: Arc<Semaphore>
}
impl RequestBuilder {
    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }
    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = client;
        self
    }
    pub fn semaphore(&mut self, semaphore: Arc<Semaphore>) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    pub fn new() -> Self {
        RequestBuilder { 
            url: String::new(),
            client: Client::new(),
            semaphore: Arc::new(Semaphore::new(5)) 
        }
    }
    pub fn build(&self) -> Request {
        Request {
            url: self.url.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone()

        }
    }
}
// Shut up clippy
impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder::new()
    }
}

