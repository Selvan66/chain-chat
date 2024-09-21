use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    middleware::Next,
    FromRequest,
};

use crate::domain::messages::*;
use crate::session::UserSession;
use crate::utils::{e500, see_other_with_flash};

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
