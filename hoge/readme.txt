echo 42 > number.txt
cargo run --bin err_panic
84
rm number.txt
cargo run --bin err_panic

echo hoge > number.txt
cargo run --bin err_panic


cargo run --bin err_string
cargo run --bin err_no_crate