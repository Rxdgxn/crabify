use reqwest::header::ACCEPT;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() {
    let cont = get("https://api.spotify.com/v1/me/player/currently-playing").await;
    let v: serde_json::Value = serde_json::from_str(&cont.unwrap()).unwrap();
    let name = v["item"]["name"].to_string();
    println!("{}", name);
}

async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer {TOKEN}")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}