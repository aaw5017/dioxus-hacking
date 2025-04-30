use tokio::{signal, sync::mpsc};
use warp::Filter;

mod api;

#[tokio::main]
async fn main() -> () {
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<bool>();

    tokio::spawn(async move {
        let _ = dotenvy::dotenv();
        let env_port = std::env::var("SERVER_PORT");

        match env_port {
            Ok(port_str) => {
                let parsed_port = port_str.parse::<u16>();

                match parsed_port {
                    Ok(port) => {
                        let public_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
                        let routes = warp::fs::dir(public_path).or(api::bind_routes());

                        println!("Server listening on {}", port);
                        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
                    }
                    Err(_) => {
                        println!("Unable to parse SERVER_PORT to u16");
                        let _ = shutdown_send.send(true);
                    }
                }
            }
            Err(_) => {
                println!("SERVER_PORT not found in ENV!");
                let _ = shutdown_send.send(true);
            }
        }
    });

    tokio::select! {
        Ok(_) = signal::ctrl_c() => {},
        Some(_) = shutdown_recv.recv() => {},
    }

    println!("All tasks finished. Shutting down");
}
