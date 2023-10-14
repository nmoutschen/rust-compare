use crate::Contains;

pub struct AhoCorasick(aho_corasick::AhoCorasick);

impl AhoCorasick {
    pub fn new(patterns: Vec<String>) -> Self {
        Self(aho_corasick::AhoCorasick::new(&patterns).unwrap())
    }
}

impl Contains for AhoCorasick {
    fn contains(&self, s: &str) -> bool {
        self.0.is_match(s)
    }
}
