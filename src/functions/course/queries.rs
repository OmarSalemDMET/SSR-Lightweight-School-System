pub async fn add_course(
    c: &super::models::AddCourse,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let num_id = uuid::Uuid::new_v4().to_string().split_off(7);
    let id = format!("{}{}", &c.c_name, &num_id);
    sqlx::query!(
        r#"
        INSERT INTO course (id, c_name, description, department_id)
        VALUES ($1, $2, $3, $4);
        "#,
        &id,
        &c.c_name,
        &c.description,
        &c.department_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn search_course(
    c: &super::models::SearchCourse,
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::FetchCourse>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::FetchCourse,
        r#"
            SELECT * FROM course WHERE c_name = $1;
        "#,
        &c.c_name
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_all_courses(
    pool: &sqlx::PgPool,
) -> Result<Vec<super::models::FetchCourse>, sqlx::Error> {
    let results = sqlx::query_as!(super::models::FetchCourse, "SELECT * FROM course;")
        .fetch_all(pool)
        .await?;
    Ok(results)
}
