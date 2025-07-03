use actix_web::{Responder, get, web::Html};
use askama::Template;

use crate::app_error::AppError;

#[derive(Template)]
#[template(path = "employee.html", block = "register_employee")]
pub struct AddEmployeeView;

#[get("/add_employee_view")]
pub async fn add_employee_view() -> Result<impl Responder, AppError> {
    Ok(Html::new(AddEmployeeView.render()?))
}

