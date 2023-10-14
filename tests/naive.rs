use rust_compare::{naive::Naive, Contains};

const BAD_BOTS: &str = include_str!("../bad_bots.txt");
const USER_AGENTS: &str = include_str!("../user_agents.txt");

#[test]
fn test_naive() {
    let bad_bots = BAD_BOTS
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let naive = Naive::new(bad_bots);

    let result = USER_AGENTS
        .split('\n')
        .map(|user_agent| naive.contains(user_agent))
        .fold(0, |acc, v| acc + v as usize);

    assert_eq!(result, 57);
}
