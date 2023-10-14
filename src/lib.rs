pub mod aho_corasick;
pub mod naive;

pub trait Contains {
    fn contains(&self, s: &str) -> bool;
}
