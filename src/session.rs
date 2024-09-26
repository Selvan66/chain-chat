use std::future::{ready, Ready};
use std::ops::Deref;

use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};

#[derive(Clone, Debug)]
pub struct UserId(pub String);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct UserSession(Session);

impl UserSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn log_out(self) {
        self.0.purge()
    }

    pub fn renew(&self) {
        self.0.renew()
    }

    pub fn insert_user_id(&self, user_id: String) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<String>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }
}

impl FromRequest for UserSession {
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<UserSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(UserSession(req.get_session())))
    }
}
