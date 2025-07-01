use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
    pub nationality: String,
    pub degree: String,
    pub gender: String,
    pub marital_status: String,
    pub address: String,
    pub constructed_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeSignUp {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub dob: chrono::NaiveDate,
    pub nationality: String,
    pub degree: String,
    pub gender: String,
    pub marital_status: String,
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct EmployeeResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub nationality: String,
}

#[derive(serde::Deserialize)]
pub struct AdditionalEmployeeDetails {
    pub employee_id: String,
    pub position: String,
    pub salary: f64,
    pub work_location: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

pub struct EmployeeDetailsPayload {
    pub position: String,
    pub salary: bigdecimal::BigDecimal,
    pub work_location: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}


#[derive(serde::Deserialize)]
pub struct EmployeePhoneNumgber {
    pub employee_id : String,
    pub phone_number : String,
}

