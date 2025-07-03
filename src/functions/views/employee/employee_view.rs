use actix_web::{web::Html, Responder};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path="employee.html", block="register_employee")]
pub struct AddEmployeeView;

pub async fn add_employee_view() -> Result<impl Responder, AppError> {
    Ok(Html::new(AddEmployeeView.render()?))
}