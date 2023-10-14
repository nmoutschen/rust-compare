use crate::Contains;

pub struct Naive(Vec<String>);

impl Naive {
    pub fn new(patterns: Vec<String>) -> Self {
        Self(patterns)
    }
}

impl Contains for Naive {
    fn contains(&self, s: &str) -> bool {
        for pattern in &self.0 {
            if s.contains(pattern) {
                return true;
            }
        }

        false
    }
}
