use models::PingResponse;
use warp::{Filter, Rejection, Reply};

pub fn bind_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let prefix = warp::path!("api" / "v1" / ..);
    let ping = warp::get()
        .and(warp::path("ping"))
        .map(|| warp::reply::json(&PingResponse::new()));

    return prefix.and(ping);
}
