use crate::network::server::Server;

pub enum ManagerMode {
    Server,
    Client,
}

pub struct NetworkManager {
    mode: ManagerMode,
}

impl NetworkManager {
    pub fn builder() -> NetworkManagerBuilder {
        NetworkManagerBuilder::new()
    }
    
    pub async fn run(&self) {
        match self.mode {
            ManagerMode::Server => {
                Server::new().run().await;
            }
            ManagerMode::Client => {
                println!("Running client");
            }
        }
    }
}

pub struct NetworkManagerBuilder {
    mode: ManagerMode,
}

impl NetworkManagerBuilder {
    pub fn new() -> Self {
        Self { mode: ManagerMode::Server }
    }
    
    pub fn enable_server(mut self) -> Self {
        self.mode = ManagerMode::Server;
        self
    }
    
    pub fn enable_client(mut self) -> Self {
        self.mode = ManagerMode::Client;
        self
    }
    
    pub fn build(self) -> NetworkManager {
        NetworkManager { mode: self.mode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_run() {
        let manager = NetworkManager::builder().enable_server().build();
        manager.run().await;
    }
}
