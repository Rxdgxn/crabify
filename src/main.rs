#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, CONTENT_LENGTH};
use serde_json::{from_str, json, Value};
use std::fs;
use eframe::{egui, App, run_native, NativeOptions};
use egui::ScrollArea;

macro_rules! read_token {
    () => {
        fs::read_to_string(".env").expect("Failed to read .env file")
    };
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let content = fs::read_to_string("pid.spid").expect("Failed to read .spid file");

    // Get user ID
    let uid_req = get("https://api.spotify.com/v1/me").await;
    let v: Value = from_str(&uid_req.unwrap()).unwrap();
    // Note: the format 1 .. x - 1 is required because of the `\TRACK\` json response
    let uid = &v["id"].to_string()[1 .. &v["id"].to_string().len()-1];
    let username = String::from(&v["display_name"].to_string()[1 .. &v["display_name"].to_string().len()-1]);


    // Get saved tracks
    let tracks_req = get("https://api.spotify.com/v1/me/tracks?limit=50").await;
    let v: Value = from_str(&tracks_req.unwrap()).unwrap();
    let saved_tracks = &v["items"];
    let mut uris = String::new();
    
    let mut tracks: Vec<(String, String)> = Vec::new(); // actual tracks list

    for i in 0 .. saved_tracks.as_array().unwrap().len() {
        let track = &saved_tracks[i]["track"];
        let uri = &track["uri"].to_string()[1 .. track["uri"].to_string().len()-1];
        let name = &track["name"].to_string()[1 .. track["name"].to_string().len()-1];
        
        let mut current_artists = String::new();
        let artists_info = track["artists"].as_array().unwrap();
        for a in artists_info {
            let artist_name = &a["name"].to_string()[1 .. a["name"].to_string().len()-1];
            current_artists.push_str(artist_name);
            current_artists.push_str(", ");
        }

        tracks.push((String::from(name), String::from(&current_artists[0 .. current_artists.len()-2])));

        uris.push_str(uri);
        uris.push(',');
    }

    
    match &content as &str {
        "" => {
            // Create playlist
            let data = json!({
                "name": &(username.clone() + &"'s Saved Songs"),
                "description": "Spotify API",
                "public": false,
            }).to_string();
            let playlist_req = post(&("https://api.spotify.com/v1/users/".to_string() + uid + "/playlists"), data).await;
            let v: Value = from_str(&playlist_req.unwrap()).unwrap();
            let pid = &v["id"].to_string()[1 .. &v["id"].to_string().len()-1];

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

    let native_options = NativeOptions::default();
    run_native("Crabify", native_options, Box::new(|cc| Box::new(MainApp::new(cc, tracks, username))))
}

// TODO: link for copying to clipboard + Track struct
#[derive(Default)]
struct MainApp {
    tracks: Vec<(String, String)>, // (track_name, artists)
    username: String
}

impl MainApp {
    fn new(_cc: &eframe::CreationContext<'_>, tracks: Vec<(String, String)>, username: String) -> Self {
        Self { tracks, username }
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Here are all of {}'s amazing songs!", self.username));
            ui.heading(""); // Funny spacing hack for now

            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    // The list of the saved tracks
                    for track in &self.tracks {
                        ui.horizontal(|ui| {
                            ui.label(format!("\"{}\" by {}", track.0, track.1));
                            if ui.button("Copy Link").clicked() {
                                println!("Copied to clipboard");
                            }
                        });
                    }
                });
        });
    }
}

// In the future, the requests should be handled better

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