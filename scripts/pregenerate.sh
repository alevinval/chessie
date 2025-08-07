cargo run --bin chessie-pregen > magic_movements.rs
mv -f magic_movements.rs src/magic/magic_movements.rs
make lint
