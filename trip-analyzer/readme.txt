#データの入手元
https://www1.nyc.gov/site/tlc/about/tlc-trip-record-data.page

#即時実行
cargo run --release -- data/yellow_tripdata_2020-02.csv

#一旦ビルド、実行
cargo build --release
target/release/trip-analyzer data/yellow_tripdata_2020-02.csv

以上