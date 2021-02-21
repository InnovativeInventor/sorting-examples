## All the sorts
An example implementation of a variety of sorting algos for fun in Rust.


## Benchmarking
Compiling and generating benchmark files (should take ~17 mins):
``` sh
cd benchmark
cargo build --release
./target/release/benchmark
```
Benchmarking:
```sh
time cat benchmark-8.txt | ./sort --bubble
```

Checksums:
```
> cd benchmark && shasum *.txt
54924f861d6c94050314249c52c5090d98a67ccf  benchmark-2.txt
51789e8e24d902ceb160c45516fac106133d264f  benchmark-3.txt
d834c3bf880477ca39b2e96fef24031e25f6af81  benchmark-4.txt
4488d51651ee94ec4b308739fe97a6666462bbfe  benchmark-5.txt
bd239972293bd521d98efdabec48d608afd92cd6  benchmark-6.txt
28a61f76c840ac330e28b6a08c27ebea9e627ad2  benchmark-7.txt
4b3654facac9ec18fd2bcd7a6cef848500851ea0  benchmark-8.txt
e4ddd39f721f7a025bed6ba19bcf21ece78fb06b  benchmark-9.txt
```
