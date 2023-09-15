cargo new cembedd

cd cembedd

cargo add --git https://github.com/huggingface/candle.git candle-core --rename candle --features "cuda"
candle_nn
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

// cargo add uuid --features v4
