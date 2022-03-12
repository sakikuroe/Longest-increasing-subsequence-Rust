# Longest Increasing Subsequence (LIS) in Rust

Find one of the strictly (or weakly) longest increasing (or decreasing) subsequence.

## Exsample

### Longest Increasing Subsequence

```rust
fn it_works() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

    assert_eq!(v.lis_strictly(), vec![1, 2, 3, 5, 7, 9]);
    assert_eq!(v.lis_weakly(), vec![1, 1, 2, 3, 5, 8, 9, 9]);
}
```

### Longest Decreasing Subsequence
```rust
fn it_works() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

    assert_eq!(v.lds_strictly(), vec![9, 6, 5, 3]);
    assert_eq!(v.lds_weakly(), vec![9, 6, 5, 3, 3]);
}
```

## Usage 
`Cargo.toml`  
```
[dependencies]
longest_increasing_subsequence_rust = { git = "https://github.com/sakikuroe/Longest-increasing-subsequence-Rust" }
```

## Performance
O(NlogN) (where N is the length of vector)

## Algorithm
Dinamic programming (DP) with binary search

## Note
- The trait Ord must be implemented for the array elements.
- If there are multiple such vectors, the vector created is not uniquely defined.