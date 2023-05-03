use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, CONTENT_LENGTH};
use serde_json::{from_str, json, Value};
use base64::{Engine as _, engine::general_purpose};
use std::fs;

macro_rules! read_token {
    () => {
        fs::read_to_string(".env").expect("Failed to read .env file")
    };
}

#[tokio::main]
async fn main() {

    let token = grab_token().await;
    fs::write(".env", token).expect("Failed to write token to .env file");

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

async fn grab_token() -> String {
    let client_id = String::from(include_str!("../id.secret").trim());
    let client_secret = String::from(include_str!("../secret.secret").trim());
    let auth = client_id + ":" + &client_secret;
    let data = json!({
        "grant_type": "client_credentials", // ??????????????????????????
    }).to_string();

    let tok_req = custom_post("https://accounts.spotify.com/api/token", data, auth).await;
    let v: Value = from_str(&tok_req.unwrap()).unwrap();
    println!("{:?}", v);
    let token = v["access_token"].to_string();
    
    token
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

// TODO: get rid of this
async fn custom_post(url: &str, data: String, auth: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(AUTHORIZATION, "Basic ".to_string() + &general_purpose::URL_SAFE_NO_PAD.encode(auth))
        .body(data)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}