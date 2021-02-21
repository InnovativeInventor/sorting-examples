## All the sorts
An example implementation of a variety of sorting algos for fun in Rust.


## Benchmarking
Compiling and generating benchmark files
``` sh
cd benchmark
cargo build --release
./target/release/benchmark
```
Benchmarking:
```
time cat benchmark-8.txt | ./sort --bubble
```

