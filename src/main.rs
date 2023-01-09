use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

#[tokio::main]
async fn main() {
    let cont = get("https://api.spotify.com/v1/me/tracks").await;
    let v: serde_json::Value = serde_json::from_str(&cont.unwrap()).unwrap();
    let saved_songs = &v["items"];
    for i in 0..saved_songs.as_array().unwrap().len() {
        let track = &saved_songs[i]["track"];
        println!("{}. {} => {}", i+1, track["name"], track["external_urls"]["spotify"]);
    }
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