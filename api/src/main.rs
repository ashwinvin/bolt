use actix_web::{body, http, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Ping {
    body: String,
}

#[actix_web::get("/ping")]
async fn ping(req: HttpRequest, body: String) -> HttpResponse {
    println!("GET {}", req.uri());

    println!("Body: \n{body}");

    println!("Headers: ");

    for (key, value) in req.headers().iter() {
        println!("{} => {}", key.as_str(), value.to_str().unwrap());
    }

    println!("------------------------------------------------------");

    let body = Ping {
        body: "pong".to_string(),
    };

    let response = HttpResponse::Ok().json(body);

    return response;
}

#[actix_web::post("/ping2")]
async fn ping2(req: HttpRequest, body: String) -> HttpResponse {
    println!("POST {}", req.uri());

    println!("Body: \n{body}");

    println!("Headers: ");

    for (key, value) in req.headers().iter() {
        println!("{} => {}", key.as_str(), value.to_str().unwrap());
    }

    println!("------------------------------------------------------");

    let body = Ping {
        body: "pong 2".to_string(),
    };

    let response = HttpResponse::Ok().json(body);

    return response;
}

async fn e404(_req: HttpRequest) -> HttpResponse {
    println!("404 was called");

    let body = body::BoxBody::new("Not Found");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::NOT_FOUND).set_body(body);

    return response;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(ping2)
            .default_service(web::post().to(e404))
    });

    let port: u16 = 2000;
    let address = "0.0.0.0";

    println!("Starting server on {} port {}", address, port);
    app.bind((address, port)).unwrap().run().await
}
