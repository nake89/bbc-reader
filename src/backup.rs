#![deny(warnings)]
use serde::Deserialize;

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let json: Ip = reqwest::get("http://httpbin.org/ip").await?.json();
    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
