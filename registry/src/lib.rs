use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

#[derive(Clone)]
pub struct AppRegistry {
    pub health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(connection_pool: ConnectionPool) -> Self {
        Self {
            health_check_repository: Arc::new(HealthCheckRepositoryImpl::new(connection_pool)),
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}