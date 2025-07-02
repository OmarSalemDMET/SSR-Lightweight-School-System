use actix_web::{Responder, web::Html};
use askama::Template;

use crate::app_error::AppError;
#[derive(Template)]
#[template(path = "course.html", block = "add_course")]
pub struct AddCourseView;

pub async fn add_course_view() -> Result<impl Responder, AppError> {
    Ok(Html::new(AddCourseView.render()?))
}
