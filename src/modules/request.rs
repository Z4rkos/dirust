use std::sync::Arc;
use tokio::sync::Semaphore;
use reqwest::Client;


#[derive(Clone)]
pub struct RequestHandler {
    pub url: String,
    client: Client,
    pub semaphore: Arc<Semaphore>
}
impl RequestHandler {
    pub async fn send(self, word: String) -> Result<String, reqwest::Error> {
        let _permit = self.semaphore.acquire().await.unwrap();
        let url = format!("{}/{}", self.url, word);
        let response = self.client.get(url).send().await?;
        let status = response.status().to_string();
        // println!("{status}");
        Ok(status)
    }
}

pub struct RequestHandlerBuilder {
    url: String,
    client: Client,
    semaphore: Arc<Semaphore>
}
impl RequestHandlerBuilder {
    pub fn new() -> Self {
        RequestHandlerBuilder { 
            url: String::new(),
            client: Client::new(),
            semaphore: Arc::new(Semaphore::new(5)) 
        }
    }

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

    pub fn build(&self) -> RequestHandler {
        RequestHandler {
            url: self.url.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone()

        }
    }
}
// Shut up clippy
impl Default for RequestHandlerBuilder {
    fn default() -> Self {
        RequestHandlerBuilder::new()
    }
}
