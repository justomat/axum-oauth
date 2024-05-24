use anyhow::Result;
use axum::{
	extract::{Path, Query},
	response::{Html, IntoResponse},
	routing::{get, get_service},
	Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

async fn handler_hello(
	(params): Query<HelloParams>,
) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello", "HANDLER");
	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("<h1>Hello, {name}</h1>"))
}

fn routes_hello() -> Router {
	let routes = Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello2));
	routes
}

#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

async fn handler_hello2(
	Path(name): Path<String>,
) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2", "HANDLER");
	Html(format!("<h1>Hello, {name}</h1>"))
}

#[tokio::main]
async fn main() -> Result<()> {
	let routes_hello = Router::new()
		.merge(routes_hello())
		.fallback_service(routes_static());

	let listener =
		TcpListener::bind("127.0.0.1:3000").await?;
	println!(
		"Listening on http://{}",
		listener.local_addr()?
	);
	axum::serve(listener, routes_hello.into_make_service())
		.await?;
	Ok(())
}

fn routes_static() -> Router {
	Router::new()
		.nest_service("/", get_service(ServeDir::new("./")))
}
