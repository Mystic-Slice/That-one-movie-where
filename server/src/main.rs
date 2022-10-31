use std::{net::{SocketAddr, IpAddr, Ipv6Addr}, str::FromStr};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use axum::{Router, routing::get};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::{CorsLayer, Any}};
use axum::http::{Method};
mod api;
use crate::api::get_movie_info;

// Setup the command line interface with clap
#[derive(Parser, Debug)]
#[clap(name = "server", about = "Server for movie info website")]
struct CmdLineOptions {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8081")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let cmd_opts = CmdLineOptions::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", cmd_opts.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/api/movie_info", get(get_movie_info))
        .merge(SpaRouter::new("/assets", cmd_opts.static_dir))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(
            CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
        );

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(cmd_opts.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        cmd_opts.port,
    ));

    log::info!("Server listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}