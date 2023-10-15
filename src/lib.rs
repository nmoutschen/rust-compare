mod ac;
pub use ac::AhoCorasick;
mod ac_dfa;
pub use ac_dfa::ACDFA;
mod naive;
pub use naive::Naive;

pub trait Contains {
    fn contains(&self, s: &str) -> bool;
}
