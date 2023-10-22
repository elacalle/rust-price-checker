mod crawler;
mod scrapper;
use crawler::client::Client;
use fantoccini;
use std::env;
mod shared;
use shared::url::is_valid_url;

use crawler::driver::Driver;
use crawler::options::Capabilities;

const DEFAULT_URL: &str = "";

#[tokio::main]
async fn main() {
    let commands: Vec<String> = env::args().collect();

    let default_url = String::from(DEFAULT_URL);
    let url = is_valid_url(commands.get(1).unwrap_or(&default_url)).unwrap();

    bot(url).await
}

async fn bot(url: &str) {
    let client = Client::create("http://localhost:4444");

    client.visit("https://www.pccomponentes.com/msi-modern-14-c13m-426xes-intel-core-i7-1355u-16gb-1tb-ssd-14").await;

    // let value = client.screenshot().await.unwrap();

    //let mut file = File::create("foo.png").unwrap();
    // file.write_all(&value);
    // file.expect("archivo no encontrado")w(&value);
}
