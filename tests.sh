cd aiyu-sorts
cargo build --release -q
echo "Aiyu benchmarks:"
time cat ../benchmark/benchmark-5.txt | ./target/release/aiyu-sorts | shasum

cd ..
echo

cd max-sorts
cargo build --release -q
echo "Max benchmarks:"
echo "Max -- default"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -d | shasum
echo "Max -- quicksort"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -q | shasum
echo "Max -- insert"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -i | shasum
echo "Max -- mergesort"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -m | shasum

echo "Default sorting algo in Linux:"
time cat ../benchmark/benchmark-5.txt | sort -g | shasum

