use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    middleware::Next,
    FromRequest, HttpMessage,
};

use crate::session::UserSession;
use crate::utils::{e500, see_other_with_flash};
use crate::{domain::messages::*, session::UserId, utils::see_other};

pub async fn reject_logged_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        UserSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(_) => {
            let response = see_other_with_flash("/user/info", USER_LOGIN);
            let e = anyhow::anyhow!("The user is log in");
            Err(InternalError::from_response(e, response).into())
        }
        None => next.call(req).await,
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        UserSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = see_other("/auth/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
