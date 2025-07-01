#[derive(serde::Serialize, serde::Deserialize)]
pub struct DepartmentRequest {
    pub d_name: String,
    pub description: String,
    pub supervisor_id: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct FetchDepartment {
    pub id: String,
    pub d_name: String,
    pub description: String,
    pub supervisor_id: String,
    pub constructed_date: chrono::NaiveDate,
    pub updated_date: chrono::NaiveDate,
}
