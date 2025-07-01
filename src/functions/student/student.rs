use crate::{app_error::AppError, functions::student::models::GradeStudentSearch};

use super::models::RegisterStudent;
use actix_web::{HttpResponse, Responder,
    post,
    web::{self, Html},
};
use askama::Template;
#[post("/register_student")]
pub async fn student_sign_up(
    pool: web::Data<sqlx::PgPool>,
    s: web::Form<RegisterStudent>,
) -> Result<impl Responder, AppError> {
    let result = super::queries::register_student(&s, pool.get_ref()).await;
    match result {
        Ok(_) => Ok(Html::new(super::templates::StudentSuccessMSG.render()?)),
        Err(e) => {
            eprintln!("Error registering student: {}", e);
            let msg = format!("Registering Student");
            let e_msg = super::templates::StudentError { message: &msg };
            Ok(Html::new(e_msg.render()?))
        }
    }
}

#[post("/register_attnedance")]
pub async fn register_attendance_handler(
    pool: web::Data<sqlx::PgPool>,
    r: web::Json<super::models::RegisterAttendance>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = super::queries::register_attendance(&r, pool.get_ref()).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Attendance registered successfully")),
        Err(e) => {
            eprintln!("Error registering attendance: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to register attendance"))
        }
    }
}

#[post("/fetch_students_by_tutorial")]
pub async fn get_stuednt_by_tutorial(
    pool: web::Data<sqlx::PgPool>,
    t: web::Json<super::models::TutorialStudentSearch>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = super::queries::search_students_tutorial(&t, pool.get_ref()).await;
    match result {
        Ok(students) => Ok(HttpResponse::Ok().json(students)),
        Err(e) => {
            eprintln!("Error fetching students by tutorial: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to fetch students by tutorial"))
        }
    }
}

#[post("/fetch_students_by_grade")]
pub async fn get_student_by_grade_handler(
    pool: web::Data<sqlx::PgPool>,
    g: web::Form<GradeStudentSearch>,
) -> Result<impl Responder, AppError> {
    let result = super::queries::fetch_students_by_grade(&g, pool.get_ref()).await;
    match result {
        Ok(students) => {
            let student_view = super::templates::StudentTemplate { students };
    
            Ok(Html::new(student_view.render()?))
        }
        Err(e) => {
            eprintln!("Error fetching students by grade: {}", e);
            Err(AppError::Render(askama::Error::Custom("Failed to fetch students by grade".into())))
        }
    }
}
