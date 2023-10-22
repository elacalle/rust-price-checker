use crate::shared::symbols::Symbol;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq)]
pub struct Capability {
    pub symbol: Symbol,
}

impl Capability {
    pub fn new(key: String, value: String) -> Capability {
        return Capability {
            symbol: Symbol { key, value },
        };
    }
}

const HEADLESS: &str = "headless";
const DISABLE_GPU: &str = "disable-gpu";
const NO_SANDBOX: &str = "no-sandbox";
const DISABLE_SHM: &str = "no-disable-dev-shm";
const HEADER: &str = "user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

const DRIVER_OPTION_KEY: &'static str = "goog:chromeOptions";

pub struct Catalog {
    pub headless: Capability,
    pub disable_gpu: Capability,
    pub no_sandbox: Capability,
    pub disable_smh: Capability,
    pub header: Capability,
}

type Value = HashMap<String, Vec<String>>;

pub struct Capabilities {
    list: HashSet<Capability>,
    arguments: Value,
}

impl Capabilities {
    pub fn catalog() -> Catalog {
        return Catalog {
            headless: Capability::new(String::from("headless"), String::from(HEADLESS)),
            disable_gpu: Capability::new(String::from("disable_gpu"), String::from(DISABLE_GPU)),
            no_sandbox: Capability::new(String::from("no_sandbox"), String::from(NO_SANDBOX)),
            disable_smh: Capability::new(String::from("disable_smh"), String::from(DISABLE_SHM)),
            header: Capability::new(String::from("header"), String::from(HEADER)),
        };
    }

    pub fn init() -> Capabilities {
        return Capabilities {
            list: HashSet::new(),
            arguments: HashMap::new(),
        };
    }

    pub fn formatted_capabities(&self) -> serde_json::Value {
        let mut arguments: Vec<String> = [].to_vec();

        for e in self.list.iter() {
            arguments.push(format!("--{}", e.symbol.value))
        }

        return serde_json::json!({ "args": arguments });
    }

    pub fn generate(self) -> serde_json::Map<String, serde_json::Value> {
        let mut capabilities = serde_json::map::Map::new();

        capabilities.insert(DRIVER_OPTION_KEY.to_string(), self.formatted_capabities());

        return capabilities;
    }

    pub fn add(&mut self, capability: Capability) -> () {
        self.list.insert(capability);
    }
}
