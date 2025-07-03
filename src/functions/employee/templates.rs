use askama::Template;

#[derive(Template)]
#[template(path = "employee.html", block = "employee_response")]
pub struct EmployeeResTemplate {
    pub employee_res: super::models::EmployeeResponse,
}

#[derive(Template)]
#[template(path = "employee.html", block = "view_all_employees")]
pub struct EmployeeViewTemplate {
    pub employees: Vec<super::models::Employee>,
}
#[derive(Template)]
#[template(path = "error_msg.html", block = "error_msg")]
pub struct EmployeeError<'a> {
    pub message: &'a str,
}
