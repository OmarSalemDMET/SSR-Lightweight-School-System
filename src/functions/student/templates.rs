use askama::Template;

#[derive(Template)]
#[template(path = "students.html", block = "students_table")]
pub struct StudentTemplate {
    pub students: Vec<super::models::FullStudentView>,
}
#[derive(Template)]
#[template(path = "success_message.html", block = "registered_student")]
pub struct StudentSuccessMSG;

#[derive(Template)]
#[template(path="error_msg.html" ,block = "error_msg")]
pub struct StudentError<'a>{
    pub message : &'a str
}