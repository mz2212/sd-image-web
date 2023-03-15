use axum::{
	routing::get,
	response::IntoResponse,
	Router
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", get(root_resp));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

	println!("Listening on: {}", addr.to_string());
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn root_resp() -> &'static str {
	"Hello World!"
}
