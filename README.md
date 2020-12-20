
## inohashmap

Stores values for strings in a Hashmap in a fast and compact way.

Good to count strings and assign ids to them or similar. Address space of string data is limited to u32::MAX (4GB).
string data is size in bytes of all uniquely inserted strings + string length metadata per string.


### Memory Consumption
Memory Consumption is lower than with a regular hashmap, 30% lower in the [compare_allocations](compare_allocations/README.md) test.


### Bench

```
Running target/release/deps/bench-f01c908733fbb7f4

running 8 tests
test tests::bench_fnv                            ... bench:     137,678 ns/iter (+/- 10,262)
test tests::bench_fnv_full                       ... bench:   5,021,742 ns/iter (+/- 251,143)
test tests::bench_hasmap                         ... bench:     118,997 ns/iter (+/- 8,068)
test tests::bench_hasmap_full                    ... bench:   4,858,801 ns/iter (+/- 242,614)
test tests::bench_hasmap_full_large_struct       ... bench:   5,634,283 ns/iter (+/- 293,666)
test tests::bench_tant_termmap                   ... bench:     136,552 ns/iter (+/- 6,591)
test tests::bench_tant_termmap_full              ... bench:   5,659,779 ns/iter (+/- 345,914)
test tests::bench_tant_termmap_full_large_struct ... bench:   5,769,806 ns/iter (+/- 371,247)
```

