mod crawler;
mod scrapper;

use crate::crawler::spider;
use crate::scrapper::parser;
use std::{env, process};

fn main() {
    let commands: Vec<String> = env::args().collect();

    match commands.get(1) {
        Some(value) => bot(value),
        None => {
            print!("INVALID_URL")
        }
    }
}

fn bot(url: &str) {
    let mut scrapper = spider::create();

    scrapper.setup();

    match scrapper.run(url) {
        Ok(body) => {
            let worker: parser::Worker = parser::create(&body);
            match worker.run() {
                Ok(value) => {
                    println!("{}", value)
                }
                Err(_e) => {
                    process::exit(1);
                }
            }
        }
        Err(_e) => {
            process::exit(1);
        }
    }
}
