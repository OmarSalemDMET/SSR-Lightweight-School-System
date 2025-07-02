use actix_web::{web::Html, Responder};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path="login.html", block="student_login")]
pub struct StudentLoginView;

pub async fn student_login_view() -> Result<impl Responder, AppError>{
    Ok(Html::new( StudentLoginView.render()?))
}


#[derive(Template)]
#[template(path="login.html", block="teacher_login")]
pub struct TeacherLoginView;

pub async fn teacher_login_view() -> Result<impl Responder, AppError>{
    Ok(Html::new( TeacherLoginView.render()?))
}




