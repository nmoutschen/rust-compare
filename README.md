# Comparison of multi-pattern string matching algorithms

## Naive implementation

This iterates over all pattern strings and uses `s.contains(pattern)` to find matches.

### Speed

```
naive                   time:   [406.07 ps 408.45 ps 411.04 ps]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  8 (8.00%) high severe
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
aho_corasick_nfa        time:   [38.056 ps 38.369 ps 38.711 ps]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  3 (3.00%) high severe
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
aho_corasick_dfa        time:   [17.762 ps 17.895 ps 18.068 ps]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
```


### Memory usage

```
dhat: Total:     4,102,437 bytes in 1,498 blocks
dhat: At t-gmax: 2,884,974 bytes in 1,283 blocks
dhat: At t-end:  0 bytes in 0 blocks
```