pub mod client;
pub mod driver;
mod mod_tests;
pub mod options;

#[derive(PartialEq)]

pub struct Bot {}

impl Bot {
    fn create() -> Self {
        return Bot {};
    }
}

// pub mod spider {
//     use std::collections::HashMap;
//     use std::fs::File;
//     use std::io::Write;
//     use std::process;
//
//     const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";
//
//     pub struct Worker {
//         user_agent: &'static str,
//         headers: HashMap<&'static str, &'static str>,
//         // browser: Browser,
//     }
//
//     impl Worker {
//         pub fn setup(&mut self) {
//             self.headers.insert("user-agent", self.user_agent);
//         }
//
//         pub fn run(&self, url: &str) -> () {}
//
//         pub fn screenshot(&self, url: &str) -> () {}
//     }
//
//     pub fn create() -> Worker {
//         Worker {
//             user_agent: &USER_AGENT,
//             headers: HashMap::new(),
//         }
//     }
// }
