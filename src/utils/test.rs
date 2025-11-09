// used in tests
#[allow(dead_code)]
pub fn tokens(s: &[&str]) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}
