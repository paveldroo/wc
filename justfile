# Watch dev files
run:
    cargo run -- -w test.txt

lint:
    cargo fmt
    cargo clippy -- -D warnings

test:
    cargo test --test integration

release:
    cargo build --bin ccwc --release
    cp target/release/ccwc .
    chmod +x ./ccwc
    ./ccwc -c test.txt

watch:
    cargo watch -q -c -w src/ -x "run -- -c test.txt"
