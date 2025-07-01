use std::env;

use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use crate::functions::assignments::models::AssignedHomeWork;

pub async fn add_course(
    a: &super::models::AddAssingment,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO work_assignment (id, course_id, description, grade, supervisor_id)
        VALUES ($1, $2, $3, $4, $5);
        "#,
        &id,
        &a.course_id,
        &a.description,
        &a.grade,
        &a.supervisor_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn assign_homework(
    a: &super::models::AssignHomeWork,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    if a.student_id.is_empty() {
        return Ok(());
    }

    let mut qb: sqlx::QueryBuilder<_> = sqlx::QueryBuilder::new(
        "INSERT INTO student_assignment (student_id, assignment_id, duedate, grade, teacher_id) ",
    );

    qb.push_values(a.student_id.iter(), |mut row, student_id| {
        row.push_bind(student_id)
            .push_bind(&a.assignment_id)
            .push_bind(&a.duedate)
            .push_bind(&a.grade)
            .push_bind(&a.teacher_id);
    });

    let query = qb.build();
    query.execute(pool).await?;
    Ok(())
}

pub async fn grade_students(
    a: &super::models::GradeStudents,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    if a.student_data.is_empty() {
        return Ok(());
    }
    let mut qb: sqlx::QueryBuilder<_> = sqlx::QueryBuilder::new(
        "INSERT INTO grade_student (student_id, assignment_id, teacher_id, student_grade) ",
    );
    qb.push_values(a.student_data.iter(), |mut row, s_data| {
        row.push_bind(&s_data.student_id)
            .push_bind(&s_data.assignment_id)
            .push_bind(&s_data.teacher_id)
            .push_bind(&s_data.student_grade);
    });
    let query = qb.build();
    query.execute(pool).await?;
    Ok(())
}

pub async fn fetch_students_grades(
    a: &super::models::SearchAssignment,
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::FetchStudentGradeData>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::FetchStudentGradeData,
        r#"
    SELECT 
    s.first_name, s.middle_name, s.last_name, 
    t.code, 
    sa.assignment_id, sa.grade
    FROM student s, student_assignment sa, tutorial t
    WHERE s.id = sa.student_id AND s.tutorial_id = t.id AND sa.assignment_id = $1;
    "#,
        a.assignment_id
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_assignments(
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::WorkAssignment>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::WorkAssignment,
        r#"
        SELECT ws.id, ws.description, c.c_name, sd.first_name, sd.middle_name, sd.last_name, sd.d_name
        FROM work_assignment ws
        JOIN course c ON ws.course_id = c.id
        JOIN supervisor_details_helper sd ON ws.supervisor_id = sd.id
        "#).fetch_all(pool).await?;
    println!("{:?}", result);
    Ok(result)
}

pub async fn fetch_assignments_for_grading(
    pool: &sqlx::PgPool,
    s: &super::models::FetchAssignmentForGrading,
) -> Result<Vec<super::models::AssignedHomeWork>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::AssignedHomeWork,
        r#"
        SELECT student_id, assignment_id, grade, duedate, teacher_id
        FROM student_assignment 
        WHERE 
        teacher_id = $1
    "#,
        s.teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn view_student_assignment_grades(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignment,
) -> Result<Vec<super::models::FetchStudentAssingmentGrades>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::FetchStudentAssingmentGrades,
        r#"
        SELECT s.first_name, s.middle_name, s.last_name,
        c.c_name, g.student_grade, stas.grade as max_grade
        FROM 
        student s, 
        grade_student g, 
        student_assignment stas, 
        work_assignment ws,
        course c
        WHERE 
        s.id = g.student_id 
        AND stas.assignment_id = g.assignment_id
        AND ws.id = g.assignment_id 
        AND ws.course_id = c.id
        AND g.assignment_id = $1
        "#,
        s.assignment_id
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_students_by_assignment(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignment,
) -> Result<Vec<super::models::FetchStudentsForGrading>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::FetchStudentsForGrading,
        r#"
        SELECT 
            s.id as student_id, s.first_name, 
            s.middle_name, s.last_name,
            c.c_name 
        FROM 
            student s
        JOIN student_assignment a ON s.id = a.student_id
        JOIN work_assignment ws ON ws.id = a.assignment_id
        JOIN course c ON c.id = ws.course_id
        WHERE a.assignment_id = $1
        "#,
        s.assignment_id
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_assignment_grades(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignment,
) -> Result<Vec<super::models::AssignmentGrade>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::AssignmentGrade,
        r#"
        SELECT 
            g.student_id,
            s.first_name,
            s.middle_name,
            s.last_name,
            g.student_grade,
            g.constructed_date
        FROM grade_student g
        JOIN student s ON g.student_id = s.id
        WHERE g.assignment_id = $1
        "#,
        s.assignment_id
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn send_assignment_grades_to_students(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignment,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = sqlx::query_as!(
        super::models::AssignmentGrade,
        r#"
        SELECT 
            g.student_id,
            s.first_name,
            s.middle_name,
            s.last_name,
            g.student_grade,
            g.constructed_date
        FROM grade_student g
        JOIN student s ON g.student_id = s.id
        WHERE g.assignment_id = $1
        "#,
        s.assignment_id
    )
    .fetch_all(pool)
    .await?;

    let from_email = env::var("OUTLOOK_EMAIL")?;
    let to_email = env::var("TO_EMAIL").unwrap_or_else(|_| "".to_string());
    let email_password = env::var("OUTLOOK_PASSWORD")?;

    let body = result
        .iter()
        .map(|grade| {
            format!(
                "Student: {} {} {}\nGrade: {}\nDate: {}\n",
                grade.first_name,
                grade.middle_name,
                grade.last_name,
                grade.student_grade,
                grade.constructed_date
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let email = Message::builder()
        .from(from_email.parse::<Mailbox>()?)
        .to(to_email.parse::<Mailbox>()?)
        .subject("Assignment Grades")
        .body(body)?;

    let creds = Credentials::new(from_email.clone(), email_password);

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
    Ok(())
}

pub async fn fetch_availble_assignments_by_grade(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignementByGrade,
) -> Result<Vec<super::models::AssignedHomeWork>, sqlx::Error> {
    let current_date = chrono::Utc::now().naive_utc().date();
    let result = sqlx::query_as!(
        super::models::AssignedHomeWork,
        r#"
        SELECT 
        student_id, 
        assignment_id, 
        grade, 
        duedate, 
        teacher_id
        FROM 
        student_assignment 
        WHERE 
        duedate > $1 AND grade = $2
    "#,
        current_date,
        s.grade
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_all_availble_assignments(
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::AssignedHomeWork>, sqlx::Error> {
    let currunt_date = chrono::Utc::now().naive_utc().date();
    let result = sqlx::query_as!(
        super::models::AssignedHomeWork,
        r#"
        SELECT 
        student_id, 
        assignment_id, 
        grade, 
        duedate, 
        teacher_id
        FROM 
        student_assignment 
        WHERE 
        duedate > $1
        "#,
        currunt_date
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_availble_assignments_by_teacher_id(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAssignmentByTeacher,
) -> Result<Vec<AssignedHomeWork>, sqlx::Error> {
    let currunt_date = chrono::Utc::now().naive_utc().date();
    let result = sqlx::query_as!(
        super::models::AssignedHomeWork,
        r#"
        SELECT 
        student_id, 
        assignment_id, 
        grade, 
        duedate, 
        teacher_id
        FROM 
        student_assignment 
        WHERE 
        teacher_id = $1
        AND 
        duedate > $2
        "#,
        s.teacher_id,
        currunt_date
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn extend_assignment_deadline(
    pool: &sqlx::PgPool,
    nd: &super::models::NewDueDate,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE 
        student_assignment
        SET
        duedate = $1
        WHERE 
        assignment_id = $2 AND teacher_id = $3 
        "#,
        &nd.duedate,
        &nd.assignment_id,
        &nd.teacher_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fetch_availble_assignments_by_students(
    pool: &sqlx::PgPool,
    s: &super::models::SearchAvailbleAssignmentsForStudents,
) -> Result<Vec<super::models::AssignedHomeWork>, sqlx::Error> {
    let currunt_date = chrono::Utc::now().naive_utc().date();
    let result = sqlx::query_as!(
        super::models::AssignedHomeWork,
        r#"
        SELECT
        student_id, 
        assignment_id, 
        grade, 
        duedate, 
        teacher_id
        FROM 
        student_assignment
        WHERE 
        student_id = $1 AND duedate > $2
        "#,
        s.student_id,
        currunt_date
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
