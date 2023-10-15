# Comparison of multi-pattern string matching algorithms

## Naive implementation

This iterates over all pattern strings and uses `s.contains(pattern)` to find matches.

### Speed

```
naive                   time:   [4.0489 µs 4.0801 µs 4.1163 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) low severe
  6 (6.00%) high mild
  7 (7.00%) high severe
```

### Memory usage

```
dhat: Total:     55,352 bytes in 654 blocks
dhat: At t-gmax: 30,872 bytes in 646 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

## Aho-Corasick with Contiguous NFA

Uses the [Aho-Corasick](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) algorithm with
a contiguous nondeterministic field automaton.

### Speed

```
aho_corasick_nfa        time:   [378.91 ns 382.08 ns 385.18 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
```

### Memory usage

```
dhat: Total:     2,143,881 bytes in 881 blocks
dhat: At t-gmax: 604,650 bytes in 654 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

## Aho-Corasick with DFA

Uses the [Aho-Corasick](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) algorithm with
a deterministic field automaton.

### Speed

```
aho_corasick_dfa        time:   [179.63 ns 180.90 ns 182.44 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe
```


### Memory usage

```
dhat: Total:     4,102,437 bytes in 1,498 blocks
dhat: At t-gmax: 2,884,974 bytes in 1,283 blocks
dhat: At t-end:  0 bytes in 0 blocks
```