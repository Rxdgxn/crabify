use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{from_str, json, Value};
use std::fs;

#[tokio::main]
async fn main() {
    let cont = get("https://api.spotify.com/v1/me").await;
    let v: Value = from_str(&cont.unwrap()).unwrap();
    let uid = &v["id"].to_string();

    let cont = post(&("https://api.spotify.com/v1/users/".to_string() + &(uid[1..uid.len()-1]) + "/playlists")).await;
    let v: Value = from_str(&cont.unwrap()).unwrap();
    println!("{}", &v["name"].to_string());

    let cont = get("https://api.spotify.com/v1/me/tracks").await;
    let v: Value = from_str(&cont.unwrap()).unwrap();
    let saved_songs = &v["items"];
    for i in 0..saved_songs.as_array().unwrap().len() {
        let track = &saved_songs[i]["track"];
        let uri = &track["uri"].to_string()[1..track["uri"].to_string().len()-1];
        let name = &track["name"].to_string()[1..track["name"].to_string().len()-1];
        println!("{}. {} => {}", i+1, name, uri);
    }
}

async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = fs::read_to_string(".env").expect("Failed");
    let body = client.get(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer ".to_string() + &token)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

async fn post(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Obviously, the data is subject to change
    // Also, the visibility remains public no matter the value for some reason
    let data = json!({
        "name": "Playlist Test",
        "description": "Spotify API",
        "public": "false"
    });

    let client = reqwest::Client::new();
    let token = fs::read_to_string(".env").expect("Failed");
    let body = client.post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer ".to_string() + &token)
        .body(data.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}