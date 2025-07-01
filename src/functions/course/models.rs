#[derive(serde::Serialize)]
pub struct FetchCourse {
    pub id: String,
    pub c_name: String,
    pub description: String,
    pub department_id: String,
    pub constructed_date: chrono::NaiveDate,
    pub updated_date: chrono::NaiveDate,
}

#[derive(serde::Deserialize)]
pub struct AddCourse {
    pub c_name: String,
    pub description: String,
    pub department_id: String,
}

#[derive(serde::Deserialize)]
pub struct SearchCourse {
    pub c_name: String,
}
