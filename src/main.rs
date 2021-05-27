#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let _hello = warp::path!("hello" / String)
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("static/test.html"));

    let static_dir = warp::path("ex").and(warp::fs::dir("./static/"));

    let route = index.or(static_dir);

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

