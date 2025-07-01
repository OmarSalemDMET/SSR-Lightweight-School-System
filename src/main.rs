pub mod functions {
    pub mod assignments;
    pub mod course;
    pub mod department;
    pub mod employee;
    pub mod student;
    pub mod teacher;
    pub mod tutorial;
    pub mod views;
}
pub mod app_error;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::web::Html;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use askama::Template;
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::Row;
use std::env;

use crate::app_error::AppError;
use crate::functions::assignments::assignments::assign_homework_handler;
use crate::functions::assignments::assignments::fetch_all_assignments;
use crate::functions::assignments::assignments::fetch_assignments_for_grading_handler;
use crate::functions::assignments::assignments::fetch_students_grades_handler;
use crate::functions::assignments::assignments::grade_students_handler;
use crate::functions::assignments::assignments::view_student_assignment_grades_handler;
use crate::functions::assignments::assignments::{
    add_assignment_handler, fetch_assignment_grades_handler, fetch_students_for_grading_handler,
    send_assignment_grades_email_handler,
};
use crate::functions::course::course::add_course_handler;
use crate::functions::course::course::fetch_all_courses_handler;
use crate::functions::course::course::search_course_handler;
use crate::functions::department::department::add_department_handler;
use crate::functions::department::department::fetch_all_departments_handler;
use crate::functions::employee::employee::add_employee;
use crate::functions::employee::employee::get_all_employees_handler;
use crate::functions::student::student::get_student_by_grade_handler;
use crate::functions::student::student::get_stuednt_by_tutorial;
use crate::functions::student::student::register_attendance_handler;
use crate::functions::student::student::student_sign_up;
use crate::functions::teacher::teacher::fetch_all_supervisors;
use crate::functions::teacher::teacher::{
    add_teacher_handler, search_teachers_by_department_handler,
};
use crate::functions::tutorial::tutorial::add_tutorial_handler;
use crate::functions::tutorial::tutorial::assign_student_handler;
use crate::functions::tutorial::tutorial::get_all_tutorials_handler;
use crate::functions::views::student::register_student::register_student_view;
use crate::functions::views::student::student_view::student_view;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[derive(Template)]
#[template(path = "base.html")]
pub struct MessageTemp;

#[get("/echo")]
async fn echo() -> Result<impl Responder, AppError> {
    let m = MessageTemp;
    Ok(Html::new(m.render()?))
}

async fn test_db(pool: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let row = sqlx::query("SELECT 2 + 2 as sum")
        .fetch_one(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let sum: i32 = row.get("sum");
    let result = format!("Sum is {}", sum);
    Ok(HttpResponse::Ok().body(result))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:pass@localhost/schooldb02".to_string());

    // Use a connection pool
    let pool = PgPool::connect(&url).await?;

    // Test a query
    let row = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;
    let sum: i32 = row.get("sum");
    println!("1 + 1 = {}", sum);
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // <-- Add this line for permissive CORS (for development)
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" %T")) // <-- Add this line
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(pool.clone()))
            .route("/test_db", web::get().to(test_db))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(add_assignment_handler)
            .service(grade_students_handler)
            .service(assign_homework_handler)
            .service(fetch_students_grades_handler)
            .service(student_sign_up)
            .service(register_attendance_handler)
            .service(get_stuednt_by_tutorial)
            .service(assign_student_handler)
            .service(add_tutorial_handler)
            .service(get_all_tutorials_handler)
            .service(get_all_employees_handler)
            .service(add_employee)
            .service(add_department_handler)
            .service(fetch_all_departments_handler)
            .service(add_course_handler)
            .service(search_course_handler)
            .service(fetch_all_courses_handler)
            .service(add_teacher_handler)
            .service(search_teachers_by_department_handler)
            .service(get_student_by_grade_handler)
            .service(fetch_all_assignments)
            .service(fetch_all_supervisors)
            .service(fetch_assignments_for_grading_handler)
            .service(fetch_students_for_grading_handler)
            .service(fetch_assignment_grades_handler)
            .service(send_assignment_grades_email_handler)
            .service(view_student_assignment_grades_handler)
            .service(student_view)
            .service(register_student_view)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
