# Crabify

Decided to have a little fun with requests in Rust, so this little **saved-tracks-grabber** for Spotify was born. It's not really meant for great things, since I'm just experimenting with reqwest and serde_json, but it's (kinda) able to share the "Liked Songs" playlist (since as far as I know, there isn't actually a way to do that in Spotify).
Terrible implementation, I know, but it does what I wanted it to do.

## Prerequisites
* The [Rust and Cargo toolchain](https://rust-lang.org/learn/get-started/)
* NodeJS and npm (preferably latest versions)

## Instructions:
* Register an app on https://developer.spotify.com/ and set the redirect URI to http://localhost:8888/callback
* Create 2 files outside the src folder, called id.secret and secret.secret (very inspired names)
* Get the client id and client secret from the app and place them in their respective files
* `npm install`
* `node script.js`
* Navigate to `localhost:8888/login`
* Close the node.js process from the terminal. The access token for the Spotify Web API should now be in the .env file
* `cargo run --release`