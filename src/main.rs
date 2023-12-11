use warp::Filter;

mod constants;
mod handlers;
mod responses;
mod routes;
mod util;
mod variants;

#[tokio::main]
async fn main() {
    println!("Cloudflare Images Dev Server: listening for requests...",);

    let routes = routes::upload_route().or(routes::image_route());

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
