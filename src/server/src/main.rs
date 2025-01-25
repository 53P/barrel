use barrel_net::initialize_listener;

#[tokio::main]
async fn main() {
    println!("Starting...");

    let listener = initialize_listener().await;
}
