use poem::{
    EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing, post,
    web::Json,
};
use serde_json::{Value, to_string_pretty};

#[handler]
fn hello(Json(data): Json<Value>) -> String {
    let value = match to_string_pretty(&data) {
        Ok(pretty_json) => format!("Received JSON data:\n{}", pretty_json),
        Err(e) => format!("Failed to format JSON: {}", e),
    };
    println!("{}", value);
    format!("Received JSON data: {:?}", value)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/api/json", post(hello)).with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("webhooks")
        .run(app)
        .await
}
