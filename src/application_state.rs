use sqlx::MySqlPool;
use arc_swap::ArcSwap;
use std::sync::Arc;
use crate::settings::Settings;
use crate::services::user::MySQLUserService;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub user_service: Arc<MySQLUserService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, pool: MySqlPool) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            user_service: Arc::new(MySQLUserService::new(pool)),
        })
    }
}