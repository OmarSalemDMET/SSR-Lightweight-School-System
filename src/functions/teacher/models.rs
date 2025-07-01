#[derive(serde::Serialize, sqlx::FromRow)]
pub struct TeacherView {
    pub id : String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub d_name: String,
}
#[derive(serde::Deserialize)]
pub struct TeacherSearch {
    pub d_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Teacher {
    pub employee_id: String,
    pub department_id: String,
}

#[derive(serde::Serialize)]
pub struct Supervisor{
    pub id : Option<String>,
    pub first_name : Option<String>,
    pub middle_name :Option<String>, 
    pub last_name : Option<String>,
    pub d_name : Option<String>
}