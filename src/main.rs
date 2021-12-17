mod cwb;
mod env;

use hyper::Server;
use routerify::RouterService;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], env::get_port()));

    let service = RouterService::new(cwb::service()).unwrap();
    let server = Server::bind(&addr).serve(service);

    println!("Service runs on: {:?}", addr);
    if let Err(e) = server.await {
        eprint!("server error: {}", e);
    }
}
