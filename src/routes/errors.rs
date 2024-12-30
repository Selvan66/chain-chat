use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{HttpRequest, HttpResponseBuilder, Result};
use anyhow::Context;
use tera::Tera;

use crate::domain::messages::*;
use crate::utils::{delete_flash_cookie, e500};

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

pub fn error_handler<T>(res: ServiceResponse<T>) -> Result<ErrorHandlerResponse<T>> {
    let status = res.status();
    let req = res.into_parts().0;
    let new_response = HttpResponseBuilder::new(status)
        .cookie(delete_flash_cookie())
        .body(render_error_page(&req, status.as_u16()).map_err(e500)?);

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, new_response).map_into_right_body(),
    ))
}
