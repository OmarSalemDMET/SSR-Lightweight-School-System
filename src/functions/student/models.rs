use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterStudent {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
    pub nationality: String,
    pub grade: i32,
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct StudentResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct RegisterAttendance {
    pub student_id: String,
}

#[derive(Serialize)]
pub struct StudentView {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub code: Option<String>,
}
#[derive(Serialize)]
pub struct FullStudentView {
    pub id : String,
    pub first_name : String,
    pub middle_name : String,
    pub last_name : String,
    pub grade : i32,
    pub dob : chrono::NaiveDate,
    pub tutorial_id : Option<String>
}

#[derive(Deserialize)]
pub struct TutorialStudentSearch {
    pub t_code: String,
}
#[derive(Deserialize)]
pub struct GradeStudentSearch{
    pub grade : i32
}