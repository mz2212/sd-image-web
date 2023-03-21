use axum::{
	http::header,
	routing::get,
	response::IntoResponse,
	Router
};
use clap::Parser;
use tower_http::services::ServeDir;
use maud::{
	DOCTYPE,
	Markup,
	html,
};

use std::{
	net::SocketAddr,
	path::Path,
	fs,
};

#[derive(Parser)]
struct Args {
	/// Location for the full size images
	#[arg(short, long, value_name = "PATH")]
	images: String,
	/// Location for the normal size images
	#[arg(short, long, value_name = "PATH")]
	thumbnails: String,
	/// Port for the server
	#[arg(short, long, value_name = "PORT")]
	port: Option<u16>,
}

#[tokio::main]
async fn main() {
	let cli = Args::parse();
	let image_path = Path::new(&cli.images);
	let thumbnail_path = Path::new(&cli.thumbnails);

	println!("Getting images from: {}", image_path.to_str().unwrap());
	let mut image_files = fs::read_dir(&image_path).unwrap()
		.map(|res| res.map(|e| e.file_name().into_string().unwrap()).unwrap())
		.collect::<Vec<_>>();
	image_files.sort();

	let app = Router::new()
		.route("/", get(move || {
			root_get(image_files)
		}))
		.route("/index.css", get(css_get))
		.nest_service("/images", ServeDir::new(&image_path))
		.nest_service("/thumbs", ServeDir::new(&thumbnail_path));

	let addr = SocketAddr::from(([127, 0, 0, 1], cli.port.unwrap_or(3000)));

	println!("Listening on: {}", addr.to_string());
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn css_get() -> impl IntoResponse {
	([(header::CONTENT_TYPE, "text/css")], r#"
		body {
			font-family: "Source Sans Variable", "Source Sans", Arial, sans-serif;
		}
	"#)
}

async fn root_get(files: Vec<String>) -> Markup {
	html! {
		(DOCTYPE)
		meta charset="utf-8";
		link rel="stylesheet" href="index.css";
		title {"Stable Diffusion Images"}
		h1 {"Stable Diffusion Images"}
		ul {
			@for image in files.into_iter() {
				li {
					img src={"/images/"(image)}{
						"/images/"(image)
					}
				}
			}
		}
	}
}