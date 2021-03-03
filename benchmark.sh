cd aiyu-sorts
cargo build --release -q
echo "Aiyu benchmarks:"
echo "Aiyu -- quicksort"
time cat ../benchmark/benchmark-5.txt | ./target/release/aiyu-sorts -q | shasum
echo "Aiyu -- mergesort"
time cat ../benchmark/benchmark-5.txt | ./target/release/aiyu-sorts -m | shasum
echo "Max -- heapsort"
time cat ../benchmark/benchmark-5.txt | ./target/release/aiyu-sorts -h | shasum

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

cd ..
echo

cd spencer-sorts
cargo build --release -q
echo "Spencer Benchmarks"
echo "Spencer -- Insertion"
time cat ../benchmark/benchmark-5.txt | ./target/release/spencer-sorts -i | shasum
echo "Spencer -- Quick"
time cat ../benchmark/benchmark-5.txt | ./target/release/spencer-sorts -q | shasum
echo "Spencer -- Radix"
time cat ../benchmark/benchmark-5.txt | ./target/release/spencer-sorts -r | shasum
echo "Spencer -- Bubble"
time cat ../benchmark/benchmark-5.txt | ./target/release/spencer-sorts -b | shasum
echo "Spencer -- Transposition"
time cat ../benchmark/benchmark-5.txt | ./target/release/spencer-sorts -t | shasum

echo "Default sorting algo in Linux:"
time cat ../benchmark/benchmark-5.txt | sort -g | shasum

