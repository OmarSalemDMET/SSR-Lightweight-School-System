use actix_web::{Error, HttpResponse, error::ErrorInternalServerError, get, post, web};
#[post("/add_teacher")]
pub async fn add_teacher_handler(
    t: web::Json<super::models::Teacher>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let sqlp = pool.get_ref();
    match super::queries::add_teacher(&t, sqlp).await {
        Ok(_) => {
            println!("Teacher added: {}", t.employee_id);
            Ok(HttpResponse::Ok().body("Teacher added successfully"))
        }
        Err(e) => {
            eprintln!("Error adding teacher: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to add teacher"))
        }
    }
}

#[post("/search_teachers_by_department")]
pub async fn search_teachers_by_department_handler(
    department: web::Json<super::models::TeacherSearch>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let sqlp = pool.get_ref();
    match super::queries::search_teachers_by_department(&department, sqlp).await {
        Ok(teachers) => Ok(HttpResponse::Ok().json(teachers)),
        Err(e) => {
            eprintln!("Error fetching teachers by department: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to fetch teachers by department"))
        }
    }
}

#[get("/fetch_all_supervisors")]
pub async fn fetch_all_supervisors(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, Error> {
    match super::queries::fetch_supervisors(pool.get_ref()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => {
            eprintln!("Error here : {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}


