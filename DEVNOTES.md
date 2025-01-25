Any time you see `AddIncrementalWrites`, it means:
```rust
[
    [reg,param[0]],
    [reg+1,param[1]],
    [reg+2,param[2]]
    ..[reg+N,param[N]]
]
```
