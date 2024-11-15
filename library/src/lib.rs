pub mod common_lib;
pub mod network;

#[cfg(test)]
mod tests {
    use crate::common_lib;
    use crate::network;
    #[test]
    pub fn test_make_ipv4_addr() {
        let addr = network::udp::UDP::make_ipv4_addr((127, 0, 0, 1), 8080);
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
        assert_eq!(addr.port(), 8080);
    }

    #[tokio::test]
    async fn test_udp_listen() {
        let result = network::udp::UDP::UdpStream::listen("127.0.0.1:8080").await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_broadcast_enable() {
        let server = network::udp::UDP::UdpStream::listen("127.0.0.1:8085")
            .await
            .unwrap();
        server.broadcast_enable(true).await;
        assert!(server.socket.broadcast().unwrap());

        server.broadcast_enable(false).await;
        assert!(!server.socket.broadcast().unwrap());
    }

    #[tokio::test]
    async fn test_udp_send_receive_game_data() {
        let player_1_addr = network::udp::UDP::make_ipv4_addr((127, 0, 0, 1), 9000);
        let player_2_addr = network::udp::UDP::make_ipv4_addr((127, 0, 0, 1), 9001);

        let mut player_1_socket =
            network::udp::UDP::UdpStream::listen(&player_1_addr.to_string())
                .await
                .unwrap();
        let mut player_2_socket =
            network::udp::UDP::UdpStream::listen(&player_2_addr.to_string())
                .await
                .unwrap();

        let command_up = bincode::serialize(&common_lib::UserCommand::Up).unwrap();
        player_1_socket
            .send(&player_2_addr, command_up)
            .await
            .unwrap();
        let received_data = player_2_socket.read().await.unwrap();
        let deserialized_data: common_lib::UserCommand = bincode::deserialize(&received_data).unwrap();
        assert_eq!(common_lib::UserCommand::Up, deserialized_data);

        let game_data: common_lib::GameData =
            common_lib::GameData::Data((1.0, 2.0), 3.0, 4.0, 5.0, 6.0);
        let serialized_data = bincode::serialize(&game_data).unwrap();

        player_2_socket
            .send(&player_1_addr, serialized_data)
            .await
            .unwrap();

        let received_data = player_1_socket.read().await.unwrap();

        let deserialized_data: common_lib::GameData = bincode::deserialize(&received_data).unwrap();

        assert_eq!(game_data, deserialized_data);
    }
}
