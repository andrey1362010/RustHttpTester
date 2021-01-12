extern crate hyper;
use std::time::{SystemTime, UNIX_EPOCH};
use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let req_count = 50000;
    let start_time = time_millis();

    for i in 0..req_count {
        let uri = "http://127.0.0.1:13265/".parse()?;
        let resp = client.get(uri).await?;
        //println!("Response: {}", resp.status());
    }
    let end_time = time_millis();
    println!("Total speed req/sec:{}", (1000 * req_count / (end_time - start_time)));
    Ok(())
}

fn time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
