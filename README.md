cargo new cembedd

cd cembedd

cargo add --git https://github.com/huggingface/candle.git candle-core --rename candle --features "cuda"

cargo add --git https://github.com/huggingface/candle.git candle-nn --rename candle_nn --features "cuda"

cargo add hf-hub --rename hf_hub --features "tokio"

cargo add serde_json

cargo add lazy_static

cargo add anyhow

cargo add tokenizers --default-features=false --features="hf-hub"

cargo add warp

cargo add serde --features derive

cargo add chrono --features serde

cargo add tokio --features full

cargo add pretty_env_logger

## build and run

cargo build --release

./target/release/cembedd

see test.http for example requests
