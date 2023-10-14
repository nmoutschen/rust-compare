# Comparison of multi-pattern string matching algorithms

## Naive implementation

This iterates over all pattern strings and uses `s.contains(pattern)` to find matches.

```
naive                   time:   [3.9867 µs 4.0044 µs 4.0232 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
```

## Aho-Corasick

Uses the [Aho-Corasick](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) algoritm to
construct a finite state machine for all patterns.

```
aho_corasick            time:   [347.61 ns 349.17 ns 350.74 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
```