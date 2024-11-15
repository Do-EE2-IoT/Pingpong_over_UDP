pub use async_std::task::spawn;
pub use bincode;
pub use serde::{Deserialize, Serialize};
pub use tokio;
pub use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum UserCommand {
    Up,
    Down,
    None,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GameData {
    Data((f32, f32), f32, f32, f32, f32),
}
