create day:
    cargo generate --path template --name {{day}}

run day part:
    cargo run --bin {{part}} --release -p {{day}}

test day part:
    cargo test -p {{day}} {{part}}