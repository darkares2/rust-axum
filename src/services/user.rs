use crate::services::model::{CreateUserRequest, User, UserStatus};
use sqlx::MySqlPool;
use time::format_description;

pub struct MySQLUserService {
    pub pool: MySqlPool,
}

impl MySQLUserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

pub trait UserService {
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;
    async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<User>;
    async fn delete_user(&self, id: i64) -> anyhow::Result<()>;
}

impl UserService for MySQLUserService {

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            "#
        );

        let format = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
                 sign:mandatory]:[offset_minute]",
        )?;

        let users = res.fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from_i32(row.status),
                created: chrono::DateTime::parse_from_rfc3339(row.created.unwrap().format(&format).unwrap().to_string().as_str()).unwrap().into(),
                updated: chrono::DateTime::parse_from_rfc3339(row.updated.unwrap().format(&format).unwrap().to_string().as_str()).unwrap().into(),
                last_login: row.last_login.map(|ll| chrono::DateTime::parse_from_rfc3339(ll.format(&format).unwrap().to_string().as_str()).unwrap().into()).or(None),
            })
            .collect();

        Ok(users)
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE id = ?
            "#,
            id
        );
        let format = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
                 sign:mandatory]:[offset_minute]",
        )?;
    
        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from_i32(row.status),
                created: chrono::DateTime::parse_from_rfc3339(row.created.unwrap().format(&format).unwrap().to_string().as_str()).unwrap().into(),
                updated: chrono::DateTime::parse_from_rfc3339(row.updated.unwrap().format(&format).unwrap().to_string().as_str()).unwrap().into(),
                last_login: row.last_login.map(|ll| chrono::DateTime::parse_from_rfc3339(ll.format(&format).unwrap().to_string().as_str()).unwrap().into()).or(None),
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Failed to get user by id: {}", id)))
    }

    async fn create_user(&self, req: CreateUserRequest) -> anyhow::Result<User> {
        let query = sqlx::query!(
            r#"
                INSERT INTO users ( username, password, status, created, updated, last_login )
                VALUES ( ?, ?, ?, NOW(), NOW(), NULL )
            "#,
            req.username,
            req.password,
            UserStatus::from(req.status)
        );

        let res = query.execute(&self.pool).await?.last_insert_id();

        let id: i64 = res
            .try_into()
            .or_else(|_| anyhow::bail!("Failed to convert user id"))?;

        let user = self.get_user_by_id(id).await?;

        Ok(user)
    }

    async fn delete_user(&self, id: i64) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = ?
            "#,
            id
        );

        let query_result = query.execute(&self.pool).await?;            

        if query_result.rows_affected() == 0 {
            anyhow::bail!("User with id {} not found", id);
        }
        Ok(())
    }
}
