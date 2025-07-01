use super::models::EmployeeSignUp;
use super::queries::employee_sign_up;
use super::queries::get_all_employees;
use actix_web::{HttpResponse, get, post, web};
use sqlx::PgPool;

#[post("/add_employee")]
pub async fn add_employee(
    e: web::Json<EmployeeSignUp>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let sqlp = pool.get_ref();
    let employee = employee_sign_up(sqlp, &e).await;
    match employee {
        Ok(emp) => {
            println!("Employee added: {}", emp.email);
        }
        Err(e) => {
            eprintln!("Error adding employee: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to add employee"));
        }
    }
    Ok(HttpResponse::Created().body("Employee added successfully"))
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
