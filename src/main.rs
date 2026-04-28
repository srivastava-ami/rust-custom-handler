use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

// Azure Functions Custom Handler entry point
#[tokio::main]
async fn main() {
    // Get port from Azure Functions environment or default to 8080
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 8080,
    };

    // Define the HTTP route matching the httpTrigger function.json
    // Route: /api/httpTrigger
    let http_example = warp::get()
        .and(warp::path("api"))
        .and(warp::path("httpTrigger"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let message = match p.get("name") {
                Some(name) => format!("Hello, {}. This HTTP triggered function executed successfully.", name),
                None => String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response."),
            };
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(message)
        });

    // Also handle POST requests to the same endpoint
    let http_example_post = warp::post()
        .and(warp::path("api"))
        .and(warp::path("httpTrigger"))
        .and(warp::body::json::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| {
            let message = match p.get("name") {
                Some(name) => format!("Hello, {}. This HTTP triggered function executed successfully.", name),
                None => String::from("This HTTP triggered function executed successfully. Pass a name in the request body for a personalized response."),
            };
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(message)
        });

    // Combine GET and POST routes
    let routes = http_example.or(http_example_post);

    println!("Starting Azure Functions Custom Handler on port {}", port);
    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}