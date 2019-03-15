use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const BASE_URL: &str = "https://i.imgur.com/";

fn generate_link() -> (String, String) {
    let mut rng = thread_rng();
    let chars = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect::<String>();
    (
        format!("{}{}.png", BASE_URL, chars),
        format!("{}.png", chars),
    )
}

fn send_to_discord_webhook(id: u64, token: &String, image_url: &String) {
    use serenity::http;
    use serenity::model::channel::Embed;

    let webhook = http::get_webhook_with_token(id, token).expect("valid webhook");

    let resources = Embed::fake(|e| e.image(image_url));

    let _ = webhook.execute(false, |w| w.embeds(vec![resources]));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let n_threads = args.get(1).unwrap().parse().expect("valid thread count");
    let webhook_id = args
        .get(2)
        .unwrap()
        .parse::<u64>()
        .expect("valid webhook id");
    let webhook_token = args.get(3).unwrap();

    let pool = threadpool::ThreadPool::new(n_threads);
    let client = reqwest::Client::builder().gzip(true).build().unwrap();

    let images_per_second = Arc::new(AtomicUsize::new(0));

    {
        let images_per_second = images_per_second.clone();

        pool.execute(move || loop {
            println!("{} images / s", images_per_second.load(Ordering::SeqCst));
            images_per_second.store(10, Ordering::SeqCst);
            thread::sleep(Duration::from_secs(1));
        });
    }

    // + 1 for statistics thread
    for _ in 0..n_threads + 1 {
        let client = client.clone();
        let images_per_second = images_per_second.clone();
        let webhook_id = webhook_id.clone();
        let webhook_token = webhook_token.clone();

        pool.execute(move || loop {
            let (link, _path) = generate_link();

            // println!("fetching {}", link);
            match client.head(&link).send() {
                Ok(resp) => {
                    images_per_second.fetch_add(1, Ordering::SeqCst);
                    // println!("received headers for {}", link);
                    match resp.content_length() {
                        Some(len) => {
                            // println!("Content-Length for {} is {}", link, len);
                            if len > 1000 {
                                /* match std::fs::File::create(&path) {
                                    Ok(mut file) => {
                                        println!("created file {}, now writing..", &path);
                                        resp.copy_to(&mut file).is_err();
                                    }
                                    Err(e) => {
                                        eprintln!("{}", e);
                                    }
                                }; */

                                println!("found valid image at {}", &link);

                                send_to_discord_webhook(webhook_id, &webhook_token, &link);
                            }
                        }
                        None => {
                            eprintln!("Content-Length absent");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        });
    }

    pool.join();
}
