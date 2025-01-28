#![allow(unused)]
use std::io::Cursor;

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub mod packets;

pub async fn initialize_listener() -> TcpListener {
    let address = "127.0.0.1:25565";
    let listener = TcpListener::bind(address).await.unwrap();

    println!("Server started on {:?}", listener.local_addr().unwrap());

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("Connection from {}", peer_addr);
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8];
    stream.read_exact(&mut buf).await;

    // get packet length
    let packet_length = buf[0] as usize;
    println!("Length: {}", packet_length);

    let mut buf = vec![0; packet_length];
    stream.read_exact(&mut buf).await;

    let mut cursor = Cursor::new(buf);
    let packet_id = packets::varint::decode_varint(&mut cursor).await.unwrap();
    println!("Packet ID: {}", packet_id);

    //handle packet
}
