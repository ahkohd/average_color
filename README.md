# average_color

A simple image average color extractor written in Rust

[![Rust](https://github.com/ahkohd/average_color/actions/workflows/rust.yml/badge.svg)](https://github.com/ahkohd/average_color/actions/workflows/rust.yml)

---

## 🔧 Usage

```rust
use average_color;
use tokio;

#[tokio::main]
fn main() {
    let paths = ["~/test1.png", "~/test2.png"];
    let results = average_color::get_averages_colors(&paths).await;
    println!("results: {:?}", results);
}
```

## 📖 Docs

- [API reference (docs.rs)](https://docs.rs/average_color/)
