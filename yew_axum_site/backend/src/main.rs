/********************************
*          IMPORT          *
*********************************/ 
// basic backend stuff
use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

 
// logging
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
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
    
    /// logging
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}
 
#[tokio::main]
async fn main() {
    // handle cli args
    let opt = Opt::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }  

    // enable console logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/api/hello", get(hello))
        .fallback_service(get(|req| async move {
            match ServeDir::new(opt.static_dir).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
         .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    
    // create server socket
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