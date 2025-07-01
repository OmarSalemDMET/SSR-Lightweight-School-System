use actix_web::{Error, HttpResponse, error::ErrorInternalServerError, get, post, put, web};
#[post("/add_assignment")]
pub async fn add_assignment_handler(
    a: web::Json<super::models::AddAssingment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::add_course(&a, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Created().body("Assignment added successfully")),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/assign_homework")]
pub async fn assign_homework_handler(
    a: web::Json<super::models::AssignHomeWork>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::assign_homework(&a, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Created().body("Homework assigned successfully")),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/grade_students_homework")]
pub async fn grade_students_handler(
    a: web::Json<super::models::GradeStudents>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::grade_students(&a, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Grades assigned successfully")),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[get("/fetch_student_homework_grades")]
pub async fn fetch_students_grades_handler(
    a: web::Json<super::models::SearchAssignment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_students_grades(&a, pool.get_ref()).await {
        Ok(grades) => Ok(HttpResponse::Ok().json(grades)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
#[get("/fetch_all_assignments")]
pub async fn fetch_all_assignments(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, Error> {
    match super::queries::fetch_assignments(pool.get_ref()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => {
            eprintln!("Error Here :\n{}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}
#[post("/fetch_assignments_for_grading")]
pub async fn fetch_assignments_for_grading_handler(
    params: web::Json<super::models::FetchAssignmentForGrading>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_assignments_for_grading(pool.get_ref(), &params).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => {
            eprintln!("{}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[post("/view_student_assignment_grades")]
pub async fn view_student_assignment_grades_handler(
    params: web::Json<super::models::SearchAssignment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::view_student_assignment_grades(pool.get_ref(), &params).await {
        Ok(grades) => Ok(HttpResponse::Ok().json(grades)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/fetch_studentss_for_grading")]
pub async fn fetch_students_for_grading_handler(
    params: web::Json<super::models::SearchAssignment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_students_by_assignment(pool.get_ref(), &params).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => {
            eprintln!("{}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[post("/fetch_assignment_grades")]
pub async fn fetch_assignment_grades_handler(
    params: web::Json<super::models::SearchAssignment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_assignment_grades(pool.get_ref(), &params).await {
        Ok(grades) => Ok(HttpResponse::Ok().json(grades)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/send_assignment_grades_email")]
pub async fn send_assignment_grades_email_handler(
    params: web::Json<super::models::SearchAssignment>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::send_assignment_grades_to_students(pool.get_ref(), &params).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Assignment grades email sent successfully")),
        Err(e) => {
            eprintln!("Email sending failed: {}", e);
            Err(ErrorInternalServerError(e))
        }
    }
}

#[post("/fetch_available_assignments_by_grade")]
pub async fn fetch_available_assignments_by_grade_handler(
    params: web::Json<super::models::SearchAssignementByGrade>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_availble_assignments_by_grade(pool.get_ref(), &params).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[get("/fetch_all_available_assignments")]
pub async fn fetch_all_available_assignments_handler(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_all_availble_assignments(pool.get_ref()).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/fetch_available_assignments_by_teacher")]
pub async fn fetch_available_assignments_by_teacher_handler(
    params: web::Json<super::models::SearchAssignmentByTeacher>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_availble_assignments_by_teacher_id(pool.get_ref(), &params).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[put("/extend_assignment_deadline")]
pub async fn extend_assignment_deadline_handler(
    params: web::Json<super::models::NewDueDate>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::extend_assignment_deadline(pool.get_ref(), &params).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Assignment deadline extended successfully")),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[post("/fetch_available_assignments_by_student")]
pub async fn fetch_available_assignments_by_student_handler(
    params: web::Json<super::models::SearchAvailbleAssignmentsForStudents>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    match super::queries::fetch_availble_assignments_by_students(pool.get_ref(), &params).await {
        Ok(assignments) => Ok(HttpResponse::Ok().json(assignments)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}


