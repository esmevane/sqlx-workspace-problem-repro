mod pg {
    use repro_core::pg;
    use sqlx::PgPool;

    #[sqlx::test]
    fn check_db(pool: PgPool) -> sqlx::Result<()> {
        pg::check_db(&pool).await?;

        Ok(())
    }
}

mod sqlite {
    use repro_core::sqlite;
    use sqlx::SqlitePool;

    #[sqlx::test]
    fn check_db(pool: SqlitePool) -> sqlx::Result<()> {
        sqlite::check_db(&pool).await?;

        Ok(())
    }
}
