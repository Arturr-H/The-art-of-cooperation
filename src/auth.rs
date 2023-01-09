//! Handles all the user-auth with the Account manager service
/* Imports */
use reqwest::blocking::Client;
use responder::Stream;
use serde_derive::Deserialize;
use crate::ACCOUNT_MANAGER_URL;

/* Structs */
#[derive(Deserialize)]
pub struct JsonResponse {
    suid: String,
    token: String
}

/* Functions */
pub fn is_user_auth(stream: &mut Stream) -> Option<JsonResponse> {
    match stream.headers.get("token") {
        Some(token) => {
            match Client::new()
                .get(ACCOUNT_MANAGER_URL.to_owned() + "profile/verify-token")
                .header("token", *token)
                .send() {
                    Ok(req) => Some(req.json::<JsonResponse>().unwrap()),
                    Err(_) => {
                        stream.respond_status(503);
                        None
                    }
                }
        },

        /* Unauthorized */
        None => {
            stream.respond_status(401);
            None
        }
    }
}
