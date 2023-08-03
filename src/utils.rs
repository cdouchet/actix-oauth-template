use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::ClientBuilder::new()
        .gzip(true)
        .build()
        .expect("Could not create HTTP client");
    pub static ref API_PORT: u16 = var("API_PORT")
        .expect("API_PORT env var must be set")
        .parse::<u16>()
        .expect("API_PORT env var must be an unsigned integer");
    pub static ref DATABASE_URL: String =
        var("DATABASE_URL").expect("DATABASE_URL env var must be set");
    pub static ref GOOGLE_CLIENT_ID: String =
        var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID env var must be set");
    pub static ref GOOGLE_CLIENT_SECRET: String =
        var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET env var must be set");
    pub static ref GOOGLE_REDIRECT_URI: String =
        var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI env var must be set");
    pub static ref APPLE_TEAM_ID: String =
        var("APPLE_TEAM_ID").expect("APPLE_TEAM_ID env var must be set");
    pub static ref APPLE_CLIENT_ID: String =
        var("APPLE_CLIENT_ID").expect("APPLE_CLIENT_ID env var must be set");
    pub static ref APPLE_REDIRECT_URI: String =
        var("APPLE_REDIRECT_URI").expect("APPLE_REDIRECT_URI env var must be set");
    pub static ref APPLE_KEY_ID: String =
        var("APPLE_KEY_ID").expect("APPLE_KEY_ID env var must be set");
}
