use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // TODO: i understand how function pointers works, but I dont' understand the async keyword.
    // TODO: and how it adds other complexities like "Future" to the functions signature.
    let my_function_pointer = greet;
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
