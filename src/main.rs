use std::error::Error;

use loadbalancer::Loadbalancer;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};

mod loadbalancer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let servers = Loadbalancer::new(vec!["localhost:8001", "localhost:8002", "localhost:8003"]);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    while let Ok((mut socket, _)) = listener.accept().await {
        let target_service = servers.get(*servers.get_server_index().await).await;
        let connection = TcpStream::connect(target_service).await;

        if connection.is_err() {
            servers.change_server().await;
            continue;
        }

        tokio::spawn(async move {
            copy_bidirectional(&mut socket, &mut connection.unwrap())
                .await
                .expect("Failed to transfer")
        });
        servers.change_server().await;
    }

    Ok(())
}
