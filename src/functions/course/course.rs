use actix_web::{error::ErrorInternalServerError, get, post, web, Error, HttpResponse};

#[post("/add_course")]
pub async fn add_course_handler(
    c: web::Json<super::models::AddCourse>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::add_course(&c, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Created().body("Successfully added course")),
        Err(e) => {
            eprintln!("Error Here : {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}
#[post("/search_course")]
pub async fn search_course_handler(
    c: web::Json<super::models::SearchCourse>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::search_course(&c, pool.get_ref()).await {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
#[get("/fetch_all_course")]
pub async fn fetch_all_courses_handler(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_all_courses(pool.get_ref()).await {
        Ok(courses) => Ok(HttpResponse::Ok().json(courses)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

