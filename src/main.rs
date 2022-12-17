use actix_web::{self, App, HttpRequest, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .service(main_endpoint)
            .service(name_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/")]
async fn main_endpoint(_req: HttpRequest) -> impl Responder {
    return "Welcome to the Rust Actix Web Template!"
}


#[actix_web::get("/{name}")]
async fn name_endpoint(req: HttpRequest) -> impl Responder {

    // Can also do this to get path parameters
    let name: &str = match req.match_info().get("name") {
        Some(name) => name,
        None => "World",
    };

    // Get the auth header
    let _auth: String = get_header(&req, "authorization");
    
    // Response String
    return format!("Hello {}!", name);
}

// The get_header() function is used to bypass
// any invalid header errors.
pub fn get_header(req: &HttpRequest, key: &str) -> String {
    return match req.headers().get(key) {
        Some(v) => v,
        None => return "".to_string(),
    }.to_str().unwrap().to_string();
}