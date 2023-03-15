use axum::{
	routing::get,
	response::IntoResponse,
	Router
};
use clap::Parser;
use tower_http::services::ServeDir;

use std::net::SocketAddr;
use std::path::Path;

#[derive(Parser)]
struct Args {
	/// Location for static assets
	#[arg(short, long, value_name = "PATH")]
	css: String,
	/// Location for the full size images
	#[arg(short, long, value_name = "PATH")]
	images: String,
	/// Location for the normal size images
	#[arg(short, long, value_name = "PATH")]
	thumbnails: String,
}

#[tokio::main]
async fn main() {
	let cli = Args::parse();
	let app = Router::new()
		.route("/", get(root_get))
		.nest_service("/css", ServeDir::new(Path::new(&cli.css)))
		.nest_service("/images", ServeDir::new(Path::new(&cli.images)))
		.nest_service("/thumbs", ServeDir::new(Path::new(&cli.thumbnails)));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

	println!("Listening on: {}", addr.to_string());
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn root_get() -> impl IntoResponse {
	"Hello World!"
}