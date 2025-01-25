#![allow(unused)]
use tokio::net::{TcpListener, TcpStream};

pub async fn initialize_listener() -> TcpListener {
    let address = "127.0.0.1:25565";
    let listener = TcpListener::bind(address).await.unwrap();

    println!("Server started on {:?}", listener.local_addr().unwrap());

    loop {
        let Ok((stream, _)) = listener.accept().await else {
            todo!()
        };
        let peer_addr = stream.peer_addr().unwrap();

        println!("Connection from {}", peer_addr);
    }
}
