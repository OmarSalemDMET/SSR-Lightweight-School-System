#[derive(serde::Deserialize)]
pub struct SearchAssignment {
    pub assignment_id: String,
}
#[derive(serde::Deserialize)]
pub struct StudentSearchAssignment {
    pub student_id: String,
}

#[derive(serde::Deserialize)]
pub struct SearchAssignementByGrade {
    pub grade: i32,
}

#[derive(serde::Deserialize)]
pub struct SearchAssignmentByTeacher {
    pub teacher_id: String,
}

#[derive(serde::Deserialize)]
pub struct NewDueDate {
    pub duedate: chrono::NaiveDate,
    pub assignment_id: String,
    pub teacher_id: String,
}
#[derive(serde::Deserialize)]
pub struct SearchAvailbleAssignmentsForStudents {
    pub student_id: String,
}

#[derive(serde::Serialize)]
pub struct FetchStudentGradeData {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub code: String,
    pub assignment_id: String,
    pub grade: i32,
}

#[derive(serde::Serialize)]
pub struct FetchStudentGrades {
    pub student_id: String,
    pub teacher_id: String,
    pub assignment_id: String,
    pub student_grade: i32,
    pub constructed_date: chrono::NaiveDate,
}

#[derive(serde::Deserialize)]
pub struct GradeSingleStudent {
    pub student_id: String,
    pub teacher_id: String,
    pub assignment_id: String,
    pub student_grade: i32,
}

#[derive(serde::Deserialize)]
pub struct GradeStudents {
    pub student_data: Vec<GradeSingleStudent>,
}

#[derive(serde::Serialize, Debug)]
pub struct WorkAssignment {
    pub id: Option<String>,
    pub c_name: Option<String>,
    pub description: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub d_name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct AddAssingment {
    pub course_id: String,
    pub description: String,
    pub grade: i32,
    pub supervisor_id: String,
}
#[derive(serde::Deserialize)]
pub struct AssignHomeWork {
    pub student_id: Vec<String>,
    pub assignment_id: String,
    pub grade: i32,
    pub duedate: chrono::NaiveDate,
    pub teacher_id: String,
}

#[derive(serde::Deserialize)]
pub struct FetchAssignmentForGrading {
    pub teacher_id: String,
}

#[derive(serde::Serialize)]
pub struct FetchStudentAssingmentGrades {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub c_name: String,
    pub student_grade: i32,
    pub max_grade: i32,
}

#[derive(serde::Serialize)]
pub struct AssignedHomeWork {
    pub student_id: String,
    pub assignment_id: String,
    pub grade: i32,
    pub duedate: chrono::NaiveDate,
    pub teacher_id: String,
}

#[derive(serde::Serialize)]
pub struct FetchStudentsForGrading {
    pub student_id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub c_name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct AssignmentGrade {
    pub student_id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub student_grade: i32,
    pub constructed_date: chrono::NaiveDate,
}

