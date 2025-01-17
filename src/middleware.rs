use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    middleware::{ErrorHandlerResponse, Next},
    FromRequest, HttpMessage, HttpRequest, HttpResponseBuilder,
};
use anyhow::Context;
use tera::Tera;

use crate::{
    domain::messages::*,
    error::e500,
    session::{UserId, UserSession},
    utils::{delete_flash_cookie, see_other, see_other_with_flash},
};

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
            tracing::debug!("User_id {} | Access granted", user_id);
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            tracing::debug!("Reject anonymouse user");
            let response = see_other("/auth/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}

fn render_error_page(req: &HttpRequest, status: u16) -> Result<String, anyhow::Error> {
    let tera = Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let mut ctx = tera::Context::new();

    if let Some(flash_cookie) = req.cookie("_flash") {
        ctx.insert("flash_message", flash_cookie.value());
    }

    if status == 404 {
        ctx.insert("error_404_message", MESSAGE_404);
    } else if status == 500 {
        ctx.insert("error_500_message", MESSAGE_500);
    }

    tera.render("error.html", &ctx)
        .context("Cannot render home page")
}

pub fn error_handler<T>(res: ServiceResponse<T>) -> actix_web::Result<ErrorHandlerResponse<T>> {
    let status = res.status();
    let req = res.into_parts().0;
    let new_response = HttpResponseBuilder::new(status)
        .cookie(delete_flash_cookie())
        .body(render_error_page(&req, status.as_u16()).map_err(e500)?);

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, new_response).map_into_right_body(),
    ))
}
