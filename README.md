# imgur-enumerator

Enumerates valid image links from imgur and can forward them to a Discord webhook, a Telegram channel and/or a file.

Thanks to Rust's asynchronous I/O with Tokio and the Hyper HTTP library,
imgur-enumerator has very high performance.

With 800 concurrent requests, it consumed 7Mbps UP and 31Mbps DOWN checking ~7000 image links per second and finding ~490 valid images per minute.

# Demo

![Demo](demo.gif)

# Compiling

Install Rust with Cargo for your system.
You can use https://rustup.rs/ or install your system packages.

```
cargo install --git https://github.com/leo-lb/imgur-enumerator
```

# Usage

`imgur-enumerator -c 50 -e imgur-links.txt`

Run `imgur-enumerator --help` for more information.
