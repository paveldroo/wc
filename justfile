# Watch dev files
run:
    cargo run -- -c test.txt

lint:
    cargo clippy

release:
    cargo build --bin ccwc --release
    cp target/release/ccwc .
    chmod +x ./ccwc
    ./ccwc -c test.txt

watch:
    cargo watch -q -c -w src/ -x "run -- -c test.txt"

test:
    cargo test -q quick_dev -- --no-capture

watch-test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --no-capture"
