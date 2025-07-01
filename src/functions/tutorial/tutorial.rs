use actix_web::{Error, HttpResponse, error::ErrorInternalServerError, get, post, put, web};

#[get("/all_tutorials")]
pub async fn get_all_tutorials_handler(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let result = super::queries::fetch_all_tutorials(pool.get_ref()).await;
    match result {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            eprintln!("Error here {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[post("/add_tutorial")]
pub async fn add_tutorial_handler(
    t: web::Json<super::models::AddTutorial>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::add_tutorial(&t, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Created().body("Added Tutorial Succesfully")),
        Err(e) => {
            eprintln!("Error here : {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[put("/assign_students_to_class")]
pub async fn assign_student_handler(
    s: web::Json<super::models::StudentTutorialReq>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::assign_student_to_tutorial(&s, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Successfully Assigned Students to Tutorial")),
        Err(e) => {
            eprintln!("Error Here : {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}
