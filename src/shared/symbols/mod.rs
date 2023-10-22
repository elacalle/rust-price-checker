#[derive(Eq, PartialEq, Hash)]
pub struct Symbol {
    pub key: String,
    pub value: String,
}

impl Symbol {
    pub fn new(key: String, value: String) -> Symbol {
        Symbol {
            key: key,
            value: value,
        }
    }
}
