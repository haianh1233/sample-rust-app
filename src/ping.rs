use std::time::Instant;

pub async fn run_ping(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let _response = reqwest::get(url).await?;
    let elapsed = start.elapsed();
    println!("Ping to {}: {}ms", url, elapsed.as_millis());
    Ok(())
}