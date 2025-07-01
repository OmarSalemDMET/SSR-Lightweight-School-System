pub async fn add_department(
    d: &super::models::DepartmentRequest,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO department (id, d_name, description, supervisor_id)
        VALUES ($1, $2, $3, $4)
        "#,
        &id,
        &d.d_name,
        &d.description,
        &d.supervisor_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fetch_all_departments(
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::FetchDepartment>, sqlx::Error> {
    let result = sqlx::query_as!(super::models::FetchDepartment, "SELECT * FROM department;")
        .fetch_all(pool)
        .await?;
    Ok(result)
}
