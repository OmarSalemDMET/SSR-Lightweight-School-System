use sqlx::PgPool;

pub async fn add_tutorial(
    t: &super::models::AddTutorial,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO tutorial 
        (id, code, grade, max_capacity, OVERALL_SCORE)
        VALUES ($1, $2, $3, $4, $5) 
        "#,
        &id,
        &t.code,
        &t.grade,
        &t.max_capacity,
        t.overall_score
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fetch_all_tutorials(
    pool: &PgPool,
) -> Result<Vec<super::models::FetchTutorial>, sqlx::Error> {
    let result = sqlx::query_as!(
        super::models::FetchTutorial,
        r#"
    SELECT * FROM tutorial;
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn assign_student_to_tutorial(
    student_tutorial: &super::models::StudentTutorialReq,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let tutorial_code = &student_tutorial.tutorial_id;
    let ids = &student_tutorial.id;
    sqlx::query!(
        "UPDATE student SET tutorial_id = $1 WHERE id = ANY($2)",
        tutorial_code,
        ids
    )
    .execute(pool)
    .await?;
    Ok(())
}
