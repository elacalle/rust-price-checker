use std::{fs::File, io::Write, collections::HashMap};

use headless_chrome::{Browser, protocol::cdp::Page};

fn main() {
    let url = "";
    let browser = Browser::default().expect("BROWSER_INIT_FAILED");

    let tab = browser.new_tab().expect("TAB_INIT_FAILED");

    let mut headers = HashMap::new();
    let mut attributes = HashMap::new();

    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36");

    tab.set_extra_http_headers(headers).expect("HEADER_ERROR");

    tab.navigate_to(url).expect("NAVIGATION_FAILED");

    tab.wait_until_navigated().expect("NAVIGATION_FAILED");

    let element = tab.find_element("#priceBlock").unwrap();
    
    let attrs = element.get_attributes().expect("ATTRS_NOT_FOUND");
    let attribute_result = attrs.expect("ATTRS_EMPTY");
    
    let attributes_amount = attribute_result.len() / 2;

    for n in 0..attributes_amount {
      let name = attribute_result.get(n * 2).expect("unknown name");
      let value = attribute_result.get(n * 2 + 1).expect("unknown value");


      print!("clave: {} y valor {}\n", name, value);

      attributes.insert(name.as_str(), value.as_str());
    }


    println!("{:?}", attributes.get("data-price").expect("UNKNOWN PRICE"));
}
