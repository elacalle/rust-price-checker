pub mod parser {
    use scraper::{Html, Selector};

    pub struct Strategy {
        
    }

    impl Strategy {
        fn call(&self, document: &scraper::Html) -> Result<String, &str> {
            let selector = Selector::parse("#priceBlock").expect("INVALID SELECTOR");

            match document.select(&selector).next() {
                Some(dom) => {
                    match dom.value().attr("data-price") {
                        Some(value) => {
                            Ok(value.to_string())
                        },
                        None => {
                            Err("price not found")
                        }
                    }
                },
                None => Err(&"selector not found")
            }
        }
    }

    pub struct Worker {
        pub document: scraper::Html,
        strategy: Strategy
    }

    impl Worker {
        pub fn run(&self) -> Result<String, &str> {
            return self.strategy.call(&self.document);
        }
    }

    pub fn create(body: &String) -> Worker {
        let document = Html::parse_document(body);
        let strategy = Strategy {};
        
        Worker { document: document, strategy: strategy }
    }
}