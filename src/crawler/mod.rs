pub mod spider {
    use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
    use headless_chrome::Browser;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::process;

    const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

    pub struct Worker {
        user_agent: &'static str,
        headers: HashMap<&'static str, &'static str>,
        browser: Browser,
    }

    impl Worker {
        pub fn setup(&mut self) {
            self.headers.insert("user-agent", self.user_agent);
        }

        pub fn run(&self, url: &str) -> Result<std::string::String, anyhow::Error> {
            let new_tab = self.browser.new_tab().unwrap();
            new_tab
                .set_extra_http_headers(self.headers.clone())
                .unwrap();

            match new_tab.navigate_to(url)?.wait_until_navigated() {
                Ok(page) => {
                    let result = page.capture_screenshot(
                        CaptureScreenshotFormatOption::Png,
                        Some(75),
                        None,
                        true,
                    )?;
                    return page.get_content();
                }
                Err(e) => return Err(e),
            }
        }

        pub fn screenshot(&self, url: &str) -> () {
            let new_tab = self.browser.new_tab().unwrap();
            new_tab
                .set_extra_http_headers(self.headers.clone())
                .unwrap();

            //match new_tab
            //    .navigate_to(url)
            //    .expect("invalid url")
            //    .wait_until_navigated()
            //{
            //    Ok(page) => {
            //        match page.capture_screenshot(
            //            CaptureScreenshotFormatOption::Png,
            //            Some(75),
            //            None,
            //            true,
            //        ) {
            //            Ok(data) => {
            //                let mut file = File::create("foo.png");
            //                file.expect("archivo no encontrado").write_all(&data);
            //            }
            //            _ => process::exit(1),
            //        }
            //    }
            //    Err(e) => process::exit(1),
            //}
        }
    }

    pub fn create() -> Worker {
        let headless_browser = Browser::default().unwrap();

        Worker {
            user_agent: &USER_AGENT,
            browser: headless_browser,
            headers: HashMap::new(),
        }
    }
}
