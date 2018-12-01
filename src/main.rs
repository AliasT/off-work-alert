extern crate hyper_tls;
extern crate timer;
extern crate chrono;
extern crate tokio;

use std::thread;
use chrono::prelude::*;
use chrono::{Utc, Local};
use tokio::timer::Interval;

use std::time::{Duration, Instant};

use hyper::{Method, Request, Body};
use hyper::Client;

use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

fn alert() {
    let https = HttpsConnector::new(4).unwrap();
    let json = r#"{"msgtype":"text","text":{"content":"喂，下班了，打卡 ！- Rust Hyper"},"at":{"atMobiles":["**********"]}}"#;
    let uri: hyper::Uri = "https://oapi.dingtalk.com/robot/send?access_token=62dd349b92c612b9f5f6f87116eea019378f91f289e64d197ae21189f2adfdd7"
        .parse().unwrap();

    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static("application/json")
    );

    let client = Client::builder().build::<_, hyper::Body>(https);

    let post = client
        .request(req)
        .and_then(|res| {
            println!("POST: {}", res.status());
            res.into_body().concat2()
        })
            // And then, if reading the full body succeeds...
        .and_then(|body| {
            // The body is just bytes, but let's print a string...
            let s = std::str::from_utf8(&body)
                .expect("httpbin sends utf-8 JSON");
            println!("body: {}", s);

            Ok(())
        })
        .map_err(|err| {
            println!("error: {}", err);
        });

    rt::run(post);
}

fn main() {
    let now_2 = Local::now();
    let target = Local.ymd(now_2.year(), now_2.month(), now_2.day()).and_hms(13, 00, 0);
    let delta = target.timestamp_millis() - now_2.timestamp_millis();
    let dura = Duration::from_millis(delta as u64);
    println!("after {:?}", delta);
    thread::sleep(dura);

    let duration = Duration::from_secs(10);
    let task = Interval::new(Instant::now(), duration)
        .for_each(|_| {
            thread::spawn(move || {
                alert()
            });
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}
