use sqlx::MySqlPool;
use crate::services::model::{CreateUserRequest, User, UserStatus};

pub struct MySQLUserService {
    pub pool: MySqlPool,
}

impl MySQLUserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}
pub trait UserService {
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;
    async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<User>;
    async fn delete_user(&self, id: i64) -> anyhow::Result<()>;
}

impl UserService for MySQLUserService {
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE id = ?
            "#,
            id
        );

        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from_i32(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                last_login: row.last_login,
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Failed to get user by id: {}", id)))
    }

    async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<User> {
        let query = sqlx::query!(
            r#"
                INSERT INTO users ( username, password, status, created, updated, last_login )
                VALUES ( ?, ?, ?, NOW(), NOW(), NULL )
            "#,
             req.username, req.password, UserStatus::from(req.status));
    
        let res = query.execute(&self.pool)
            .await?
            .last_insert_id();
    
        let id: i64 = res.try_into().or_else(|_| anyhow::bail!("Failed to convert user id"))?;
    
        let user = self.get_user_by_id(id).await?;
    
        Ok(user)
    }

    async fn delete_user(&self, id: i64) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = ?
            "#,
            id);
    
        query.execute(&self.pool).await?;
    
        Ok(())
    }
    
}