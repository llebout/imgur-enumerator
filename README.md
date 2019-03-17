# imgur-enumerator

Enumerates valid image links from imgur and forwards them to a Discord webhook.
It also writes the links to a local file.

Thanks to Rust's asynchronous I/O with Tokio and the Hyper HTTP library,
imgur-enumerator has very high performance.

With 800 concurrent requests, it consumed 7Mbps UP and 31Mbps DOWN,
checking ~7000 image links per second and finding between 3 and 7 valid image links per second.

# Demo

![Demo](demo.gif)

# Compiling

Install Rust with Cargo for your system.
You can use https://rustup.rs/ or install your system packages.

```
git clone https://github.com/leo-lb/imgur-enumerator
cd imgur-enumerator
cargo build --release
```

Your built binary is now in `target/release/imgur-enumerator`

# Usage

`imgur-enumerator -c 50 -e imgur-links.txt`

Run `imgur-enumerator --help` to for more information.
