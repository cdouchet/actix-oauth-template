use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct GoogleOauthPayload {
    pub code: Option<String>,
    pub scope: Option<String>,
    pub authuser: Option<String>,
    pub prompt: Option<String>,
    pub state: Option<String>,
}

impl GoogleOauthPayload {
    pub fn is_all_none(&self) -> bool {
        if self.code.is_none()
            && self.scope.is_none()
            && self.authuser.is_none()
            && self.prompt.is_none()
            && self.state.is_none()
        {
            return true;
        }
        false
    }

    pub fn has_none(&self) -> &Option<String> {
        if self.code.is_none() {
            return &self.code;
        }
        if self.scope.is_none() {
            return &self.scope;
        }
        if self.authuser.is_none() {
            return &self.authuser;
        }
        if self.prompt.is_none() {
            return &self.prompt;
        }
        if self.state.is_none() {
            return &self.state;
        }
        &None
    }
}

#[derive(Serialize)]
pub struct GooglePostPayload {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct GoogleTokenPayload {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct GoogleJWTHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

#[derive(Deserialize, Serialize)]
pub struct GoogleJWTClaims {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub at_hash: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub iat: String,
    pub exp: String,
}

#[derive(Deserialize)]
pub struct GoogleIdentityResponse {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

#[derive(Deserialize)]
pub struct GoogleTokenResponse {
    pub error: String,
    pub error_description: String,
}
