// NOTE: this Javascript part is required to get an access token. In the future i will switch fully to Rust
// This is according to their tutorial: https://developer.spotify.com/documentation/web-api/tutorials/code-flow

let fs = require('fs');
let client_id = fs.readFileSync('./id.secret', 'utf-8');
let client_secret = fs.readFileSync('./secret.secret', 'utf-8');
let request = require('request');
let express = require('express');
let querystring = require('querystring');

const port = 8888;
let redirect_uri = `http://127.0.0.1:${port}/callback`;
let app = express();

function generateRandomString(length) {
  let text = '';
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < length; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

app.get('/login', function(_req, res) {
  let state = generateRandomString(16);
  let scope = 'playlist-modify-public playlist-modify-private user-library-read user-library-modify';
  res.cookie('spotify_auth_state', state);

  res.redirect('https://accounts.spotify.com/authorize?' +
    querystring.stringify({
      response_type: 'code',
      client_id: client_id,
      scope: scope,
      redirect_uri: redirect_uri,
      state: state
    }));
});

app.get('/callback', function(req, res) {
  let code = req.query.code || null;
  let state = req.query.state || null;

  if (state === null) {
    res.redirect('/#' +
      querystring.stringify({
        error: 'state_mismatch'
      }));
  } else {
    let authOptions = {
      url: 'https://accounts.spotify.com/api/token',
      form: {
        code: code,
        redirect_uri: redirect_uri,
        grant_type: 'authorization_code'
      },
      headers: {
        'Authorization': 'Basic ' + (new Buffer.from(client_id + ':' + client_secret).toString('base64'))
      },
      json: true
    };

    request.post(authOptions, function(error, response, body) {
      if (!error && response.statusCode === 200) {
        let token = body.access_token;
        fs.writeFileSync('./.env', token);
      }
      else {
        console.log('Something went wrong:\n' + error);
      }
    });

  }
});

app.listen(port, () => {
  console.log(`Listening at localhost:${port}/login`);
});