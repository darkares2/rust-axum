use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Active = 1,
    Blocked = 2,
}

impl UserStatus {
    pub fn from_i32(value: i32) -> UserStatus {
        match value {
            1 => UserStatus::Active,
            2 => UserStatus::Blocked,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn from(value: UserStatus) -> i32 {
        match value {
            UserStatus::Active => 1,
            UserStatus::Blocked => 2,
        }
    }
}


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    Draft = 1,
    Published = 2,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}