use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{
    db::Pool,
    external::google::{
        GoogleIdentityResponse, GoogleOauthPayload, GooglePostPayload, GoogleTokenPayload,
        GoogleTokenResponse,
    },
    handlers::error::Error,
    models::user::User,
    utils::{GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REDIRECT_URI, HTTP_CLIENT},
};

pub async fn google_auth(
    pool: Data<Pool>,
    params: web::Query<GoogleOauthPayload>,
) -> Result<HttpResponse, Error> {
    let has_none = params.has_none();
    if has_none.is_some() {
        return Err(Error::internal_server_error());
    }
    let google_post_payload = GooglePostPayload {
        code: params.code.clone().unwrap(),
        client_id: GOOGLE_CLIENT_ID.to_string(),
        client_secret: GOOGLE_CLIENT_SECRET.to_string(),
        grant_type: String::from("authorization_code"),
        redirect_uri: GOOGLE_REDIRECT_URI.to_string(),
    };
    let client = &HTTP_CLIENT;
    let google_response = match client
        .post("https://oauth2.googleapis.com/token")
        .body(serde_json::to_string(&google_post_payload).unwrap())
        .header("Content-Type", "application/json")
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Connection", "keep-alive")
        .send()
        .await
    {
        Ok(e) => e,
        Err(_) => return Err(Error::expired_token()),
    };
    if google_response.status() != reqwest::StatusCode::OK {
        let response_body = google_response.json::<GoogleTokenResponse>().await.unwrap();
        return Err(Error::google_error(
            response_body.error,
            response_body.error_description,
        ));
    }
    let response_body = google_response.text().await.unwrap();
    let parsed_body = serde_json::from_str::<GoogleTokenPayload>(&response_body).unwrap();
    let user_info_response = match client
        .get(format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
            parsed_body.access_token
        ))
        .send()
        .await
    {
        Ok(e) => e,
        Err(_) => return Err(Error::internal_server_error()),
    };

    let google_identity = serde_json::from_str::<GoogleIdentityResponse>(
        user_info_response.text().await.unwrap().as_ref(),
    )
    .unwrap();

    let conn = &mut pool.get().unwrap();

    

    Ok(HttpResponse::Ok().into())
}
