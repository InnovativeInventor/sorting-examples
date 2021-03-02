cd aiyu-sorts
cargo build --release -q
echo "Aiyu benchmarks:"
time cat ../benchmark/benchmark-5.txt | ./target/release/aiyu-sorts > /dev/null

cd ..
echo

cd max-sorts
cargo build --release -q
echo "Max benchmarks:"
echo "Max -- default"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -d > /dev/null
echo "Max -- quicksort"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -q > /dev/null
echo "Max -- insert"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -i > /dev/null
echo "Max -- mergesort"
time cat ../benchmark/benchmark-5.txt | ./target/release/max-sorts -m > /dev/null

echo "Default sorting algo in Linux:"
time cat ../benchmark/benchmark-5.txt | sort -g > /dev/null

