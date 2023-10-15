use rust_compare::{AhoCorasick, Contains};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const BAD_BOTS: &str = include_str!("../bad_bots.txt");
const USER_AGENTS: &str = include_str!("../user_agents.txt");

#[test]
fn test_aho_corasick() {
    let _profiler = dhat::Profiler::new_heap();

    let bad_bots = BAD_BOTS
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let aho_corasick = AhoCorasick::new(bad_bots);

    let result = USER_AGENTS
        .split('\n')
        .map(|user_agent| aho_corasick.contains(user_agent))
        .fold(0, |acc, v| acc + v as usize);

    assert_eq!(result, 57);
}
