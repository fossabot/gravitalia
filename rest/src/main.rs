use warp::Filter;
use dotenv;
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let connect = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("oauth2"))
        .and(warp::path::end())
        .and_then(router::get_connect);

    let routes = connect;

    warp::serve(routes)
        .run(([127, 0, 0, 1], dotenv::var("PORT").unwrap().parse::<u16>().unwrap()))
        .await;
}