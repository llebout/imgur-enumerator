use futures::{stream, Future, Stream};
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio;

const BASE_URL: &str = "https://i.imgur.com/";

struct UriGenerator {
    base_url: String,
    extension: String,
}

impl UriGenerator {
    fn new(base_url: String, extension: String) -> Self {
        Self {
            base_url,
            extension,
        }
    }
}

impl Iterator for UriGenerator {
    type Item = Uri;

    fn next(&mut self) -> Option<Uri> {
        Some(
            format!(
                "{}{}{}",
                self.base_url,
                iter::repeat(())
                    .map(|()| thread_rng().sample(Alphanumeric))
                    .take(7)
                    .collect::<String>(),
                self.extension
            )
            .parse()
            .unwrap(),
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let n_concurrent: usize = args.get(1).unwrap().parse().unwrap();
    let id: u64 = args.get(2).unwrap().parse().unwrap();
    let token: String = args.get(3).unwrap().parse().unwrap();

    let (tx, rx) = mpsc::channel::<Uri>();

    thread::spawn(move || {
        use serenity::http;
        use serenity::model::channel::Embed;

        let webhook = http::get_webhook_with_token(id, &token).expect("valid webhook");

        for uri in rx {
            let uri_str = format!(
                "{}{}",
                uri.authority_part().unwrap(),
                uri.path_and_query().unwrap()
            );

            println!("found valid image at {}", uri_str);

            let resources = Embed::fake(|e| e.image(uri_str));

            let _ = webhook.execute(false, |w| w.embeds(vec![resources]));
        }
    });

    let images_per_second = Arc::new(AtomicUsize::new(0));

    {
        let images_per_second = images_per_second.clone();
        thread::spawn(move || loop {
            println!("{} images / s", images_per_second.load(Ordering::SeqCst));
            images_per_second.store(10, Ordering::SeqCst);
            thread::sleep(Duration::from_secs(1));
        });
    }

    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);

    let images = UriGenerator::new(BASE_URL.to_string(), ".png".to_string());

    let work = stream::iter_ok(images)
        .map(move |uri| {
            client
                .request(Request::head(uri.clone()).body(Body::empty()).unwrap())
                .map(move |res| (res, uri))
        })
        .buffer_unordered(n_concurrent)
        .and_then(move |(res, uri)| {
            images_per_second.fetch_add(1, Ordering::SeqCst);

            if let Some(content_length) = res.headers().get("content-length") {
                if let Ok(content_length) = content_length.to_str() {
                    if let Ok(content_length) = content_length.parse::<usize>() {
                        if content_length > 503 {
                            tx.send(uri).unwrap();
                        }
                    }
                }
            }
            res.into_body().concat2()
        })
        .for_each(|_body| Ok(()))
        .map_err(|e| {
            eprintln!("{}", e);
        });

    tokio::run(work);
}
