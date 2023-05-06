var fs = require('fs');
var client_id = fs.readFileSync('./id.secret', 'utf-8');
var client_secret = fs.readFileSync('./secret.secret', 'utf-8');
var request = require('request');
var express = require('express');
var querystring = require('querystring');
var redirect_uri = 'http://localhost:8888/callback';
var app = express();
var port = 8888;

function generateRandomString(length) {
  let text = '';
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < length; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

app.get('/login', function(req, res) {
  var state = generateRandomString(16);
  var scope = 'playlist-modify-public playlist-modify-private user-library-read user-library-modify';
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
  var code = req.query.code || null;
  var state = req.query.state || null;

  if (state === null) {
    res.redirect('/#' +
      querystring.stringify({
        error: 'state_mismatch'
      }));
  } else {
    var authOptions = {
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
        var token = body.access_token;
        fs.writeFileSync('./.env', token);
      }
      else {
        console.log('Something went wrong:\n' + error);
      }
    });

  }
});

app.listen(port, () => {
  console.log('Listening at localhost:' + port);
});