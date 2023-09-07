/*
Challenge: Implement a Simple Concurrent Web Crawler

Your task is to build a simple web crawler in Rust that can crawl multiple web pages concurrently. You'll need to use Rust's concurrency features, such as threads and channels, to accomplish this. Here are the steps to follow:

    Create a list of URLs that you want to crawl. You can start with a small list for testing.

    Write a function that can fetch the content of a given URL using a library like reqwest.

    Create a thread pool with a fixed number of worker threads. You can use Rust's std::thread::spawn for this.

    Use channels (std::sync::mpsc) to communicate between the main thread and the worker threads. The main thread should send URLs to be crawled to the worker threads.

    Each worker thread should take a URL from the channel, fetch its content, and parse it to find more URLs to crawl. Send these URLs back to the main thread through another channel.

    Implement proper synchronization mechanisms to ensure that URLs are not crawled more than once and that the crawler doesn't run indefinitely.

    print the crawled URLs or save them to a file for later analysis.

    Add error handling to gracefully handle network errors, invalid URLs, and other potential issues.
*/

use regex::Regex;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

#[tokio::main]
async fn main() {
    let mut active_workers = 0;

    let urls = Arc::new(Mutex::new(vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.mozilla.org".to_string(),
        "https://github.com".to_string(),
        "https://www.example.com".to_string(),
        "https://en.wikipedia.org/wiki/Web_crawler".to_string(),
    ]));

    let (request_tx, request_rx) = tokio::sync::mpsc::channel(32);
    let (response_tx, mut response_rx) = tokio::sync::mpsc::channel(32);

    for url in urls.lock().unwrap().iter() {
        active_workers += 1;
        request_tx.send(url.to_string()).await.unwrap();
    }

    tokio::spawn(async move {
        worker(request_rx, response_tx).await;
    });

    while active_workers > 0 {
        let new_urls = match response_rx.recv().await {
            Some(resp) => resp,
            None => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
        };
        let mut urls = urls.lock().unwrap(); // Lock for mutable access
        active_workers -= 1;
        for url in new_urls {
            if urls.contains(&url) {
                continue;
            }
            active_workers += 1;

            urls.push(url.clone()); // Clone the String to take ownership

            request_tx.send(url.clone()).await.unwrap();

            eprintln!("Active Workers: {}", active_workers);
            println!("{:?}", url);
        }
    }
}

async fn worker<'a>(mut request_rx: Receiver<String>, response_tx: Sender<Vec<String>>) {
    while let Some(url) = request_rx.recv().await {
        let tx = response_tx.clone();
        tokio::spawn(async move {
            let new_urls = find_sub_urls(&url).await;
            tx.send(new_urls).await.unwrap();
        });
    }
}

async fn find_sub_urls(url: &str) -> Vec<String> {
    let content = match fetch_content(&url).await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Unable to read from {}: {}", url, err);
            "Error".to_string()
        }
    };

    let res = parse_content(content);

    res
}

async fn fetch_content(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url.to_string()).await?.text().await
}

fn parse_content(content: String) -> Vec<String> {
    //Check only for urls related to "rust" so this eventually finishes without scraping the entire web
    let re = Regex::new(r#"(https?://[^\s<>"']*rust[^\s<>"']*)"#).unwrap();
    re.find_iter(content.as_str())
        .map(|m| m.as_str().to_string())
        .collect()
}
