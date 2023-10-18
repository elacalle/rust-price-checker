mod crawler;
mod scrapper;
use fantoccini;
use std::env;
mod shared;
use shared::errors::validation::{self, InvalidValue};

#[tokio::main]
async fn main() {
    let commands: Vec<String> = env::args().collect();

    //bot("").await;

    // is_valid_url(commands.get(1).unwrap_or(""))
}

fn is_valid_url(url: &str) -> Result<&str, InvalidValue> {
    return if url.len() != 0 {
        Ok(url)
    } else {
        Err(validation::InvalidValue::new("test"))
    };
}

async fn bot(url: &str) {
    println!("CRATE SPIDER");

    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({
        "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage", "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"],
    });
    caps.insert("goog:chromeOptions".to_string(), opts);

    let c = fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:4444")
        .await
        .expect("Error");

    c.goto("https://www.pccomponentes.com/msi-modern-14-c13m-426xes-intel-core-i7-1355u-16gb-1tb-ssd-14").await;

    let value = c.screenshot().await.unwrap();

    //let mut file = File::create("foo.png").unwrap();
    // file.write_all(&value);
    // file.expect("archivo no encontrado")w(&value);
}
