#[cfg(test)]

mod bot {
    use super::super::Bot;

    #[test]
    fn create_returns_a_bot() {
        assert!(Bot {} == Bot::create());
    }
}
