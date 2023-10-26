use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::Parser;

use curl::easy::{Easy, List};

async fn handle_any(
    data: web::Data<Args>,
    path: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let mut response = Vec::new();
    let mut handle = Easy::new();

    let authorization_header = match req.headers().get("authorization") {
        Some(f) => f.to_str().unwrap_or(""),
        None => "",
    };

    let path = data.proxy_url.to_owned() + &path;

    println!("path: {}", path);

    handle.url(&path).unwrap();

    if !authorization_header.is_empty() {
        let mut headers = List::new();
        headers
            .append(&format!("authorization: {}", authorization_header))
            .unwrap();

        handle.http_headers(headers).unwrap();
    }
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                response.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Access-Control-Allow-Headers", "authorization"))
        .insert_header((
            "Access-Control-Allow-Methods",
            "GET,HEAD,PUT,PATCH,POST,DELETE",
        ))
        .body(response)
}

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Url to the connection
    #[arg(long)]
    proxy_url: String,

    /// Default port to be used
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Args::parse()))
            .service(web::resource("/{path:.*}").to(handle_any))
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}
