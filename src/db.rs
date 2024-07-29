use sqlx::PgPool;

pub(crate) async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!().run(pool).await.expect("Migrations failed");
}
