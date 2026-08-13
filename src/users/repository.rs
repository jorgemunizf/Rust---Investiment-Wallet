use sqlx::PgPool;
use uuid::Uuid;

use super::model::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            name,
            email,
            password_hash,
            created_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            id,
            name,
            email,
            password_hash
        )
        VALUES ($1,$2,$3,$4)
        RETURNING
            id,
            name,
            email,
            password_hash,
            created_at
        "#,
        id,
        name,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
