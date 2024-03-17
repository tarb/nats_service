use super::Repository;

pub struct Postgres {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl Postgres {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}

impl Repository for Postgres {
    type Error = sqlx::Error;

    async fn increment(&self, amount: i32) -> Result<i32, Self::Error> {
        let record = sqlx::query!("
            INSERT INTO counts (integer_value)
            VALUES (coalesce((SELECT integer_value from counts ORDER BY created_at DESC LIMIT 1), 0) + $1)
            RETURNING integer_value",
            amount
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.integer_value.unwrap_or_default())
    }

    async fn decrement(&self, amount: i32) -> Result<i32, Self::Error> {
        let record = sqlx::query!("
            INSERT INTO counts (integer_value)
            VALUES (coalesce((SELECT integer_value from counts ORDER BY created_at DESC LIMIT 1), 0) - $1)
            RETURNING integer_value", 
            amount
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.integer_value.unwrap_or_default())
    }

    async fn count(&self) -> Result<i32, Self::Error> {
        let record = sqlx::query!(
            "
        SELECT integer_value FROM counts 
        ORDER BY created_at DESC 
        LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.integer_value.unwrap_or_default())
    }
}
