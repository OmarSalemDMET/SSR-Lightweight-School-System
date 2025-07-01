use actix_web::{get, web::Html, Responder};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path="register_student.html", block="register_student_form")]
pub struct RegisterStudent;

#[get("/register_student_view")]
pub async fn register_student_view() -> Result<impl Responder, AppError>{
    Ok(Html::new(RegisterStudent.render()?))
}