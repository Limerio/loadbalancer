use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub struct Loadbalancer {
    servers: Arc<Mutex<Vec<&'static str>>>,
    server_index: Arc<Mutex<usize>>,
}

impl Loadbalancer {
    pub fn new(iter: Vec<&'static str>) -> Self {
        Self {
            servers: Arc::new(Mutex::new(iter)),
            server_index: Arc::new(Mutex::new(2)),
        }
    }

    pub async fn get_servers(&self) -> MutexGuard<'_, Vec<&'static str>> {
        self.servers.lock().await
    }

    pub async fn get_server_index(&self) -> MutexGuard<'_, usize> {
        self.server_index.lock().await
    }

    pub async fn get(&self, value: usize) -> &str {
        self.get_servers().await.get(value).unwrap()
    }

    pub async fn change_server(&self) {
        let new_server_index =
            (*self.get_server_index().await + 1) % self.get_servers().await.len();
        *self.get_server_index().await = new_server_index
    }
}
