use game::game_pingpong::{game_pingpong_run, pingpong_get_cmd};
use library::{
    common_lib::{
        self, bincode, spawn,
        tokio::{self, select},
        GameData, Receiver, Sender, UserCommand,
    },
    network::udp::UDP,
};

use std::io;
mod game;

#[tokio::main]
async fn main() -> Result<(), io::ErrorKind> {
    let my_addr = UDP::make_ipv4_addr((0, 0, 0, 0), 8080);
    let peer_addr = UDP::make_ipv4_addr((172, 16, 100, 250), 8080);
    let mut socket = UDP::UdpStream::listen(&my_addr.to_string()).await.unwrap();
    
    let (tx, rx): (Sender<UserCommand>, Receiver<UserCommand>) = tokio::sync::mpsc::channel(100);
    let (tx_game_data, mut rx_game_data): (
        common_lib::Sender<GameData>,
        common_lib::Receiver<GameData>,
    ) = tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx, tx_game_data);
    });

    loop {
        tokio::select! {
        data = socket.read() => {
                         match data{
                            Ok(cmd) => {
                           if let Ok(data) = bincode::deserialize(&cmd) {
                               if let Err(e) = pingpong_get_cmd(tx.clone(), data).await{
                                     println!("Error while send cmd to game {e}");
                                }
                                } else {
                                    println!("Invalid data");
                                    continue;
                                };

                            },
                            Err(e)  => {println!("{e}");
                            continue; },
                         }

                    },

         Some(game_data) = rx_game_data.recv()  => {
                  if let Ok(data) = bincode::serialize(&game_data){
                    if let Err(e) = socket.send(&peer_addr, data).await{
                       println!("{:?}", e);
                      }
                  }
        },
                }
    }
    Ok(())
}
