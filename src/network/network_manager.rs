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
}

pub struct NetworkManagerBuilder {
    mode: ManagerMode,
}

impl NetworkManagerBuilder {
    pub fn new() -> Self {
        unimplemented!();
        Self { mode: ManagerMode::Server }
    }
    
    pub fn enable_server(&mut self) {
        self.mode = ManagerMode::Server;
    }
    
    pub fn enable_client(&mut self) {
        self.mode = ManagerMode::Client;
    }
    
    pub fn build(self) -> NetworkManager {
        NetworkManager { mode: self.mode }
    }
}
