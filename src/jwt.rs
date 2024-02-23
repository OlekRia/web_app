use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

pub struct JwtToken {
    pub message: String,
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Ready<Result<JwtToken, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = JwtToken {
                    message: data.to_str().unwrap().to_string(),
                };
                ok(token)
            }
            None => {
                let token = JwtToken {
                    message: String::from("nothing found"),
                };
                ok(token)
            }
        }
    }
}
