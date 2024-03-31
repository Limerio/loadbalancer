use std::error::Error;

use loadbalancer::Loadbalancer;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};

mod cli;
mod loadbalancer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::generate().get_matches();
    let port = matches
        .get_one::<String>("port")
        .expect("Something went wrong with the port");
    let servers_args = matches
        .get_many::<String>("servers")
        .expect("Unable to find a list of servers seperate by commas")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let servers = Loadbalancer::new(servers_args);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

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
