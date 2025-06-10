use crate::network::network_manager::NetworkManager;

//The Game Engine is responsible for managing the game state and updating the game world.
// It also based on the provided mode enables the user to 
// 1. Host the game session
// 2. Join the game session
// This reduces the complexity and the need for a separate server
pub struct GameEngine {
    game_space: GameSpace,
    player: Player,
    game_mode: GameMode,
    server: NetworkManager, // either handles a server or a client
    ui: UserInterface
}


pub enum GameMode {
    Host {
        session_id: String,
        connected_client: Client,
    },
    Client {
        host_address: SocketAddr,
        connection: ClientConnection,
    },
    Local,
}

// obstacles streaming start/stop etc
pub struct GameSpace {
    
}
