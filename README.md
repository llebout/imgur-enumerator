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

Example: `imgur-enumerator -c 50 -e imgur-links.txt`

```
imgur-enumerator 0.1

USAGE:
    imgur-enumerator [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrent <concurrent>    Maximum amount of concurrent requests at a time [default: 4]
    -e, --export <export_file>       Path to file where found links will be written
    -k, --tg-channel <tg_channel>    Telegram Channel ID
    -l, --tg-token <tg_token>        Telegram Bot Token
    -u, --user-agent <user_agent>    Value of User-Agent header which will be used in all requests [default: Mozilla/5.0
                                     (Windows NT 10.0; Win64; x64; rv:65.0) Gecko/20100101 Firefox/65.0]
    -i, --id <webhook_id>            Discord Webhook ID
    -t, --token <webhook_token>      Discord Webhook Token
```
