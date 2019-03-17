# imgur-enumerator

Enumerates valid image links from imgur and forwards them to a Discord webhook.
It also writes the links to a local file.

Thanks to Rust's asynchronous I/O with Tokio and the Hyper HTTP library,
imgur-enumerator has very high performance.

With 800 concurrent requests, it consumed 7Mbps UP and 31Mbps DOWN,
checking ~7000 image links per second and finding between 3 and 7 valid image links per second.

# Usage

imgur n_concurrent_requests webhook_id webhook_token save_links_file
