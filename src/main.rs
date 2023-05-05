use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, CONTENT_LENGTH};
use serde_json::{from_str, json, Value};
use std::fs;

macro_rules! read_token {
    () => {
        fs::read_to_string(".env").expect("Failed to read .env file")
    };
}

#[tokio::main]
async fn main() {

    let content = fs::read_to_string("pid.spid").expect("Failed to read .spid file");

    // Get user ID
    let uid_req = get("https://api.spotify.com/v1/me").await;
    let v: Value = from_str(&uid_req.unwrap()).unwrap();
    let uid = &v["id"].to_string()[1..&v["id"].to_string().len()-1];
    let username = &v["display_name"].to_string()[1..&v["display_name"].to_string().len()-1];


    // Get saved tracks
    let tracks_req = get("https://api.spotify.com/v1/me/tracks?limit=50").await;
    let v: Value = from_str(&tracks_req.unwrap()).unwrap();
    let saved_tracks = &v["items"];
    let mut uris = String::new();
    for i in 0 .. saved_tracks.as_array().unwrap().len() {
        let track = &saved_tracks[i]["track"];
        let uri = &track["uri"].to_string()[1..track["uri"].to_string().len()-1];
        let name = &track["name"].to_string()[1..track["name"].to_string().len()-1];
        println!("{}. {} => {}", i+1, name, uri);
        uris.push_str(uri);
        uris.push(',');
    }

    
    match &content as &str {
        "" => {
            // Create playlist
            let data = json!({
                "name": &(username.to_string() + &"'s Saved Songs"),
                "description": "Spotify API",
                "public": false,
            }).to_string();
            let playlist_req = post(&("https://api.spotify.com/v1/users/".to_string() + uid + "/playlists"), data).await;
            let v: Value = from_str(&playlist_req.unwrap()).unwrap();
            let pid = &v["id"].to_string()[1..&v["id"].to_string().len()-1];

            // Add tracks to playlist and save ID
            fs::write("pid.spid", pid).expect("Failed to write to .spid file");
            let add_req = post(&("https://api.spotify.com/v1/playlists/".to_string() + pid + "/tracks?uris=" + &uris), String::new()).await;
            let v: Value = from_str(&add_req.unwrap()).unwrap();
            println!("{}", v.to_string());
        },
        id => {
            // Replace the tracks
            let replace_req = put(&("https://api.spotify.com/v1/playlists/".to_string() + id + "/tracks?uris=" + &uris)).await;
            let v: Value = from_str(&replace_req.unwrap()).unwrap();
            println!("{}", v.to_string());
        }
    }
}

async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = read_token!();
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

async fn put(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = read_token!();
    let body = client.put(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer ".to_string() + &token)
        .body("Content-Length: ".to_string() + &CONTENT_LENGTH.to_string())
        .send()
        .await?
        .text()
        .await?;
    
    Ok(body)
}

async fn post(url: &str, data: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = read_token!();
    let body = client.post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, "Bearer ".to_string() + &token)
        .body(data)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}