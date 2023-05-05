use crate::modules::request::Request;


pub struct Executor;
impl Executor {
    pub async fn run(request: Request, max_concurrent_requests: usize, wordlist: Vec<String>) {
        for word in wordlist {
            let request = request.clone();

            let fut = request.send(word.clone());
            tokio::spawn(async move {
                match fut.await {
                    Ok(res) => match res {
                        ref res if res == "200 OK" => {
                            println!("/{}", {word});
                        }
                        _ => ()
                    },
                    Err(e) => println!("{}", e)
                }
            });
        }
        loop {
            tokio::task::yield_now().await;
            if request.semaphore.available_permits() == max_concurrent_requests { break; }
        }
    }

}
