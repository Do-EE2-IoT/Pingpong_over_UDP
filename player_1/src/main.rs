use crate::key_event::get_input_command;
use game::game_pingpong::game_pingpong_run;
use library::{
    common_lib::{
        self, bincode, spawn,
        tokio::{self, select, time::error::Error},
        GameData, Receiver, Sender, UserCommand,
    },
    network::udp::UDP,
};

use std::io;
mod game;
mod key_event;

#[tokio::main]
async fn main() -> Result<(), io::ErrorKind> {
    let my_addr = UDP::make_ipv4_addr((0, 0, 0, 0), 8080);
    let peer_addr = UDP::make_ipv4_addr((172, 16, 100, 197), 8080);
    let mut socket = UDP::UdpStream::listen(&my_addr.to_string()).await.unwrap();
    let (tx, rx): (common_lib::Sender<GameData>, common_lib::Receiver<GameData>) =
        tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx);
    });
    println!("OK");
    loop {
        tokio::select! {
           result = get_input_command()=> {
                 match result {
                    Ok(UserCommand::Down) => {
                      let data = bincode::serialize(&UserCommand::Down).unwrap();
                      socket.send(&peer_addr, data).await.unwrap();
                    },
                    Ok(UserCommand::Up) => {
                        let data = bincode::serialize(&UserCommand::Up).unwrap();
                        socket.send(&peer_addr, data).await.unwrap();
                    },
                    Ok(UserCommand::None) => {
                    },
                    Err(_) => println!(),

                 }
           },

           get_game_data = socket.read()=>{
            match get_game_data{
                 Ok(data) => {
                     if let Ok(data) = bincode::deserialize(&data){
                         if let Err(e) = tx.send(data).await{
                            println!("{:?}", e);
                         }
                     }
                 },
                 Err(e) => println!("{:?}", e)
            }

           },
        }
    }

    Ok(())
}
