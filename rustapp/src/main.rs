use std::collections::HashMap;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4}; // Added SocketAddrV4
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    // 1. Decouple Port Logic
    let port: u16 = env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .map(|val| val.parse().unwrap_or(8080))
        .unwrap_or(8080);

    // 2. Define standard routes (Architecture of the API)
    let api_path = warp::path("api").and(warp::path("httpTrigger"));

    let http_get = warp::get()
        .and(api_path)
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let name = p.get("name").map(|s| s.as_str()).unwrap_or("Guest");
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(format!("Hello, {}. GET success.", name))
        });

    let http_post = warp::post()
        .and(api_path)
        .and(warp::body::json::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let name = p.get("name").map(|s| s.as_str()).unwrap_or("Guest");
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(format!("Hello, {}. POST success.", name))
        });

    let routes = http_get.or(http_post);

    // 3. Independent Binding Strategy
    // Using Ipv4Addr::UNSPECIFIED (0.0.0.0) is the key for containers.
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    
    println!("Sovereign Engine active on {}", addr);
    
    warp::serve(routes).run(addr).await
}