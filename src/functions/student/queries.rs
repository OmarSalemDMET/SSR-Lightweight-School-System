use super::models::FullStudentView;
use super::models::RegisterAttendance;
use super::models::RegisterStudent;
use super::models::StudentResponse;
use super::models::StudentView;
use super::models::TutorialStudentSearch;
use bcrypt::DEFAULT_COST;
use sqlx::PgPool;

pub async fn register_attendance(r: &RegisterAttendance, pool: &PgPool) -> Result<(), sqlx::Error> {
    let id = &r.student_id;
    let log_date = chrono::Utc::now().date_naive();
    sqlx::query!(
        r#"
        INSERT INTO student_attendance (student_id, log_date)
        VALUES ($1, $2);
        "#,
        id,
        log_date
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn register_student(
    s: &RegisterStudent,
    pool: &PgPool,
) -> Result<StudentResponse, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let password_hashed = match bcrypt::hash(&s.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return Err(sqlx::Error::InvalidArgument(format!(
                "a hashing problem {}",
                e
            )));
        }
    };
    sqlx::query!(
        r#"
        INSERT INTO student
        (id,
        first_name,
        middle_name,
        last_name,
        dob,
        nationality,
        grade)
        Values ($1, $2, $3, $4, $5, $6, $7);
        "#,
        &id,
        &s.first_name,
        &s.middle_name,
        &s.last_name,
        &s.dob,
        &s.nationality,
        &s.grade
    )
    .execute(pool)
    .await?;
    let email = format!("{}@student.mail.edu", s.user_name);

    sqlx::query!(
        r#"
        INSERT INTO student_email
        (
        username,
        email,
        student_password,
        student_id)
        Values ($1, $2, $3, $4);
        "#,
        &s.user_name,
        &email,
        &password_hashed,
        &id
    )
    .execute(pool)
    .await?;
    let student_response = StudentResponse {
        first_name: s.first_name.clone(),
        last_name: s.last_name.clone(),
        email: email.clone(),
        message: String::from("You are Successfully Registerd"),
    };
    Ok(student_response)
}

pub async fn search_students_tutorial(
    tutorial_code: &TutorialStudentSearch,
    pool: &PgPool,
) -> Result<Vec<StudentView>, sqlx::Error> {
    let rows = sqlx::query_as!(
        StudentView,
        "SELECT * FROM classroom_detail($1);",
        &tutorial_code.t_code
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn fetch_students_by_grade(
    grade: &super::models::GradeStudentSearch,
    pool: &PgPool,
) -> Result<Vec<FullStudentView>, sqlx::Error> {
    let result = sqlx::query_as!(
        FullStudentView,
        r#"
            SELECT id, first_name, 
            middle_name, last_name,
            grade, dob, tutorial_id FROM student WHERE grade = $1;
        "#,
        &grade.grade
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
