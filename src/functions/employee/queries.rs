use crate::functions::employee::models::EmployeeDetailsPayload;

use super::models::Employee;
use super::models::EmployeeResponse;
use super::models::EmployeeSignUp;
use bcrypt::DEFAULT_COST;
use sqlx::PgPool;

pub async fn employee_sign_up(
    pool: &PgPool,
    e: &EmployeeSignUp,
) -> Result<EmployeeResponse, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let password_hashed = match bcrypt::hash(&e.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return Err(sqlx::Error::InvalidArgument(format!(
                "a hashing problem {}",
                e
            )));
        }
    };

    let middle_name = e.middle_name.as_deref().unwrap_or("");

    sqlx::query!(
        r#"
        INSERT INTO employee (
            id, first_name, middle_name, last_name,
            dob, nationality, degree, gender,
            marital_status, address, constructed_date, updated_date
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW()
        )
        "#,
        &id,
        &e.first_name,
        &middle_name,
        &e.last_name,
        &e.dob,
        &e.nationality,
        &e.degree,
        &e.gender,
        &e.marital_status,
        &e.address,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
            INSERT INTO employee_email (
                employee_id, email, password, mail_code
            ) VALUES ($1, $2, $3, $4)
        "#,
        &id,
        &e.email,
        &password_hashed,
        &(String::from("@employee"))
    )
    .execute(pool)
    .await?;
    let e_response = EmployeeResponse {
        first_name: e.first_name.clone(),
        last_name: e.last_name.clone(),
        email: e.email.clone(),
        nationality: e.nationality.clone(),
    };
    Ok(e_response)
}

pub async fn get_all_employees(pool: &PgPool) -> Result<Vec<Employee>, sqlx::Error> {
    let employees = sqlx::query_as::<_, Employee>(
        r#"
        SELECT * FROM employee;
"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(employees)
}

pub async fn add_additional_informatation(
    pool: &sqlx::PgPool,
    e: &super::models::AdditionalEmployeeDetails,
) -> Result<(), sqlx::Error> {
    let salary = bigdecimal::BigDecimal::parse_bytes(e.salary.to_string().as_bytes(), 10)
        .expect("Failed to parse salary to BigDecimal");
    let payload = EmployeeDetailsPayload {
        position: e.position.clone(),
        work_location: e.work_location.clone(),
        salary: salary,
        start_date: e.start_date,
        end_date: e.end_date,
    };
    sqlx::query!(
        r#"
        INSERT INTO employee_details (
            employee_id, position, salary, work_location,
            start_date, end_date
        ) VALUES (
            $1, $2, $3, $4, $5, $6
        )
        "#,
        &e.employee_id,
        &payload.position,
        &payload.salary,
        &payload.work_location,
        &payload.start_date,
        &payload.end_date
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_employee_phone_number(
    pool: &sqlx::PgPool,
    e: &super::models::EmployeePhoneNumgber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT 
        INTO 
        employee_phonenumber
        (employee_id, phone_number) 
        VALUES
        ($1, $2)
        "#,
        &e.employee_id,
        &e.phone_number
    )
    .execute(pool)
    .await?;
    Ok(())
}
