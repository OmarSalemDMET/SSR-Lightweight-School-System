use actix_web::web::Html;
use actix_web::{Responder, get};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path = "students.html", block = "StudentForm")]
pub struct StudentForm;

#[get("/view_students")]
pub async fn student_view() -> Result<impl Responder, AppError> {
    let s = StudentForm;
    Ok(Html::new(s.render()?))
}
