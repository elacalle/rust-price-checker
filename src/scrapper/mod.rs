use headless_chrome::{protocol::cdp::Page};

mod scrapper {
    struct Browser {

    }

    fn create() -> Browser {
        let headless_browser = headless_chrome::Browser::default().expect("BROWSER_INIT_FAILED");

        Browser {}
    }
}