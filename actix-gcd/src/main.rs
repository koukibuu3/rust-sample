use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

mod gcd_module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(gcd)
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 3000)).expect("error binding server to address")
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[post("/gcd")]
async fn gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!(
            "The greatest common divider of the numbers {} and {} is <b>{}</b>\n",
            form.n, form.m, gcd_module::gcd(form.n, form.m)
        );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
