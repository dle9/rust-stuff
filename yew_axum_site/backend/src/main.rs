/********************************
*            IMPORTS            *
*********************************/ 
// basic backend stuff
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

// logging
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/********************************
*          PRETTY CLI           *
*********************************/ 
#[derive(Parser, Debug)]
#[clap(name = "server", about = "backend for my site")]
struct Opt {
    /// backend addr
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    addr: String,

    /// backend port
    #[clap(short = 'p', long = "port", default_value = "5000")]
    port: u16,
    
    // logging
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
}

/********************************
*             MAIN              *
*********************************/ 
#[tokio::main]
async fn main() {
    
    /****************
    *     SETUP     *
    *****************/ 
    // parse command line args
    let opt = Opt::parse();

    // handle logging (-l) arg
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    // enable console logging
    tracing_subscriber::fmt::init();

    /****************
    *      APP      *
    *****************/ 
    // create the server
    let app = Router::new()
        .route("/", get(hello))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    let server_socket = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    // start listening for clients   
    axum_server::Server::bind(server_socket)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
    log::info!("listening on http://{}", server_socket);

}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}