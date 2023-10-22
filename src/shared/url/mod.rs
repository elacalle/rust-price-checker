use super::errors::validation::InvalidValue;
mod mod_tests;

pub fn is_valid_url(url: &str) -> Result<&str, InvalidValue> {
    return if url.len() != 0 {
        Ok(url)
    } else {
        Err(InvalidValue::new("test"))
    };
}
