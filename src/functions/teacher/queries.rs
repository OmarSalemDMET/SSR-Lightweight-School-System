pub async fn add_teacher(
    t: &super::models::Teacher,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
            INSERT INTO teacher (id, employee_id, department_id)
            VALUES($1, $2, $3);
        "#,
        &id,
        &t.employee_id,
        &t.department_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn search_teachers_by_department(
    department: &super::models::TeacherSearch,
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::TeacherView>, sqlx::Error> {
    let query = r#"
        SELECT * FROM teacher_view_details t WHERE t.d_name = $1;
    "#;
    let result = sqlx::query_as::<_, super::models::TeacherView>(query)
        .bind(&department.d_name)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn fetch_supervisors(
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::Supervisor>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::Supervisor,
        r#"
    SELECT * FROM supervisor_details_helper;
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
