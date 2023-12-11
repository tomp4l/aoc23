cargo build --release

time for i in $(seq 1 25); do
    echo day $i:
    ./target/release/aoc23 $i
done
