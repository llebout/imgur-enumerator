use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

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
    let pool = threadpool::ThreadPool::new(50);
    let client = reqwest::Client::builder().gzip(true).build().unwrap();

    let args: Vec<String> = std::env::args().collect();

    for _ in 0..args.get(1).unwrap().parse().expect("valid thread count") {
        let client = client.clone();
        let args = args.clone();

        pool.execute(move || loop {
            let (link, _path) = generate_link();

            // println!("fetching {}", link);
            match client.get(&link).send() {
                Ok(resp) => {
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

                                send_to_discord_webhook(
                                    args.get(2).unwrap().parse().expect("valid webhook id"),
                                    args.get(3).unwrap(),
                                    &link,
                                );
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
