


Compares allocations between inohashmap, tantivity interal used hashmap and fnv



### DHAT Profile
```
➜  compare_allocations git:(main) ✗ cargo run --bin inohash     
Compiling inohashmap v0.1.0 (/home/pascal/Development/insert_only_string_hash_map)
Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/inohash`
dhat: Total:     7,813,120 bytes in 150 blocks
dhat: At t-gmax: 524,288 bytes in 3 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
➜  compare_allocations git:(main) ✗ cargo run --bin tant_termmap
   Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/tant_termmap`
dhat: Total:     28,672,320 bytes in 150 blocks
dhat: At t-gmax: 2,359,328 bytes in 5 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
➜  compare_allocations git:(main) ✗ cargo run --bin fnv         
   Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/fnv`
dhat: Total:     11,219,930 bytes in 137,850 blocks
dhat: At t-gmax: 865,262 bytes in 7,171 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html

```
