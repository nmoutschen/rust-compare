use crate::Contains;

pub struct ACDFA(aho_corasick::AhoCorasick);

impl ACDFA {
    pub fn new(patterns: Vec<String>) -> Self {
        Self(
            aho_corasick::AhoCorasick::builder()
                .kind(Some(aho_corasick::AhoCorasickKind::DFA))
                .match_kind(aho_corasick::MatchKind::LeftmostFirst)
                .build(&patterns)
                .unwrap(),
        )
    }
}

impl Contains for ACDFA {
    fn contains(&self, s: &str) -> bool {
        self.0.is_match(s)
    }
}
