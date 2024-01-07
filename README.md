# Crabify

Decided to have a little fun with requests in Rust, so this little **saved-tracks-grabber** for Spotify was born. It's not really meant for great things, since I'm just experimenting with reqwest and serde_json, but it's (kinda) able to share the "Liked Songs" playlist (since as far as I know, there isn't actually a way to do that in Spotify).
Terrible implementation, I know, but it does what I wanted it to do.

Special thanks to [Brittany Chiang](https://www.newline.co/courses/build-a-spotify-connected-app/implementing-the-authorization-code-flow) for showing how to request an OAuth token.

Instructions:
* Run `node script.js`
* Navigate to `localhost:8888/login`
* Close the Node process from the terminal