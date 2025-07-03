use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpResponse, get, post, web};

#[post("/add_department")]
pub async fn add_department_handler(
    req: web::Json<super::models::DepartmentRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let result = super::queries::add_department(&req, pool.get_ref()).await;
    match result {
        Ok(_) => Ok(HttpResponse::Created().body("Successfully added department")),
        Err(e) => {
            eprintln!("Here is an Error {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[get("/fetch_all_departments")]
pub async fn fetch_all_departments_handler(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let result = super::queries::fetch_all_departments(pool.get_ref()).await;
    match result {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            eprintln!("Error here {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}


