pub async fn check_db<'a, Executor>(executor: Executor) -> sqlx::Result<()>
where
    Executor: sqlx::PgExecutor<'a>,
{
    sqlx::query!(r#"select 1 as result"#)
        .fetch_one(executor)
        .await?;

    Ok(())
}
