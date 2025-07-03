use crate::app_error::AppError;

use super::models::EmployeeSignUp;
use super::queries::employee_sign_up;
use super::queries::get_all_employees;
use actix_web::Responder;
use actix_web::web::Html;
use actix_web::{HttpResponse, get, post, web};
use askama::Template;
use sqlx::PgPool;

#[post("/add_employee")]
pub async fn add_employee(
    e: web::Form<EmployeeSignUp>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {
    let sqlp = pool.get_ref();
    let employee = employee_sign_up(sqlp, &e).await;
    match employee {
        Ok(emp) => Ok(Html::new(
            super::templates::EmployeeResTemplate { employee_res: emp }.render()?,
        )),
        Err(e) => {
            eprintln!("Error : {}", e);
            Ok(Html::new(
                super::templates::EmployeeError {
                    message: "Error Happened While Registering Employee",
                }
                .render()?,
            ))
        }
    }
}

#[get("/get_all_employees")]
pub async fn get_all_employees_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let sqlp = pool.get_ref();
    let employees = get_all_employees(sqlp).await;
    match employees {
        Ok(emp_list) => {
            let response = web::Json(emp_list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Error fetching employees: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to fetch employees"))
        }
    }
}
