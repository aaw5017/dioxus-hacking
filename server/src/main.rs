use warp::Filter;

mod api;

#[tokio::main]
async fn main() {
    let config = config::get();
    let public_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    println!("{}", public_path);
    let routes = warp::fs::dir(public_path).or(api::bind_routes());

    warp::serve(routes)
        .run(([127, 0, 0, 1], config.server_port))
        .await;
}
