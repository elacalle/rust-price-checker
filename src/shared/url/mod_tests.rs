#[cfg(test)]

mod url {
    use super::super::is_valid_url;

    #[test]
    fn check_url_hasnt_content() {
        assert!(is_valid_url("").is_err())
    }

    #[test]
    fn check_url_has_valid_format() {
        assert!(is_valid_url("https://google.es").is_ok())
    }

    #[test]
    fn returns_same_url() {
        let url = "https://google.es";

        assert_eq!(is_valid_url(url).unwrap(), url)
    }
}
