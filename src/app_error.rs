use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::web::Html;
use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Responder for AppError {
    type Body = String;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            message: String,
        }

        let tmpl = Tmpl {
            message: self.to_string(),
        };
        if let Ok(body) = tmpl.render() {
            (Html::new(body), self.status_code()).respond_to(req)
        } else {
            (String::from("Internal Server Error"), self.status_code()).respond_to(req)
        }
    }
}
