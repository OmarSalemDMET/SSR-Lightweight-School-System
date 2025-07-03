use actix_web::{Responder, web::Html};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path = "department.html", block = "add_department")]
pub struct AddDepartmentView;

pub async fn add_department_view() -> Result<impl Responder, AppError> {
    Ok(Html::new(AddDepartmentView.render()?))
}
