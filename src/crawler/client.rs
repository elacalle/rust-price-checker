use fantoccini::error;

use super::{driver::Driver, options::Capabilities};

pub struct Client {
    driver: Driver,
}

impl Client {
    pub fn create(url: &str) -> Client {
        let catalog = Capabilities::catalog();
        let mut capabilities = Capabilities::init();

        capabilities.add(catalog.disable_gpu);
        capabilities.add(catalog.disable_smh);
        capabilities.add(catalog.headless);
        capabilities.add(catalog.no_sandbox);
        capabilities.add(catalog.header);

        let driver = Driver::new("http://localhost:4444", capabilities);

        return Client { driver };
    }

    pub async fn visit(self, url: &str) -> Result<(), error::CmdError> {
        return self.driver.client.goto(url).await;
    }
}
