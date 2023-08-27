use scraper::{Html, Selector};

mod scrapper;

use crate::scrapper::spider::create;

fn main() {
  let mut scrapper = create();
  scrapper.setup();
  
  match scrapper.run("") {
    Ok(body) => {
      let document = Html::parse_document(&body);
      let selector = Selector::parse("#priceBlock").expect("INVALID SELECTOR");

      let html = document.select(&selector).next().unwrap();

      print!("{}", html.value().attr("data-price").unwrap())

    },
    Err(_e) => print!("SCRAPPER_ERROR")
  }
}
