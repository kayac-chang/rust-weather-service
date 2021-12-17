mod cwb;
mod env;

use hyper::Server;
use routerify::RouterService;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = RouterService::new(cwb::service()).unwrap();
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprint!("server error: {}", e);
    }
}
