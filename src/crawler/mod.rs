pub mod spider {
    use std::collections::HashMap;
    use headless_chrome::Browser;

    const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

    pub struct Worker {
        user_agent: &'static str,
        headers: HashMap<&'static str, &'static str>,
        browser: Browser,
    }

    impl Worker {
        pub fn setup(& mut self) {
            self.headers.insert("user-agent", self.user_agent);
        }

        pub fn run(& self, url: &str) -> Result<std::string::String, anyhow::Error> {
            let new_tab = self.browser.new_tab().unwrap();
            new_tab.set_extra_http_headers(self.headers.clone()).unwrap();
   
            match new_tab.navigate_to(url)?.wait_until_navigated() {
                Ok(page) => {
                    return page.get_content();
                }
                Err(e) => return Err(e)
            }
        }
    }

    pub fn create() -> Worker {
        let headless_browser = Browser::default().expect("BROWSER_INIT_FAILED");

        Worker {
            user_agent: &USER_AGENT,
            browser: headless_browser,
            headers: HashMap::new()
        }
    }
}