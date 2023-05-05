var fs = require('fs');
var client_id = fs.readFileSync('./id.secret', 'utf-8');
var client_secret = fs.readFileSync('./secret.secret', 'utf-8');
var request = require('request');

var authOptions = {
  url: 'https://accounts.spotify.com/api/token',
  headers: {
    'Authorization': 'Basic ' + (new Buffer.from(client_id + ':' + client_secret).toString('base64'))
  },
  form: {
    grant_type: 'client_credentials',
    scope: 'playlist-modify-public playlist-modify-private user-read-private user-top-read user-library-read user-library-modify'
    // Hopefully these are all of the necessary scopes, yet the program keeps running into a 'server error' from the API (500)
  },
  json: true
};

request.post(authOptions, function(error, response, body) {
  if (!error && response.statusCode === 200) {
    var token = body.access_token;
    console.log(token);
  }
  else {
    console.log('Something went wrong:\n' + error);
  }
});
