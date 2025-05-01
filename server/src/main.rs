use tokio::{signal, sync::mpsc};
use warp::Filter;

mod api;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const SERVER_PORT: &str = env!("SERVER_PORT");

#[tokio::main]
async fn main() -> () {
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<bool>();

    tokio::spawn(async move {
        let parsed_port = SERVER_PORT.parse::<u16>();

        match parsed_port {
            Ok(port) => {
                let public_path = format!("{}/public", MANIFEST_DIR);
                let routes = warp::fs::dir(public_path).or(api::bind_routes());

                println!("Server listening on {}", port);
                warp::serve(routes).run(([127, 0, 0, 1], port)).await;
            }
            Err(_) => {
                eprintln!("Unable to parse SERVER_PORT to u16");
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
