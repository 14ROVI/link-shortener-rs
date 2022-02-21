use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{fs, time::{SystemTime, UNIX_EPOCH}, ops::AddAssign};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;



#[derive(Deserialize, Debug)]
struct JsonUrl {
    redirect: String,
    password: String
}



#[post("/shorten")]
async fn shorten(
    pool: web::Data<Pool>,
    data: web::Json<JsonUrl>
) -> impl Responder {

    if data.password != env::var("LINK_SHORTENER_PASSWORD").unwrap() {
        return HttpResponse::Unauthorized().finish();
    }

    let alpha_numbs = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut uri_end = String::from("");
    while time > 0 {
        uri_end.add_assign(&alpha_numbs[((time % 62) as usize)..((time % 62 + 1) as usize)]);
        time = time / 62;
    }

    let uri_start: String = thread_rng()
         .sample_iter(&Alphanumeric)
         .take(7)
         .map(char::from)
         .collect();

    let uri = uri_start + &uri_end;

    let con = pool.get().await.unwrap();
    con.execute("insert into links (uri, redirect) values ($1, $2)", &[&uri, &data.redirect]).await.unwrap();

    HttpResponse::Ok().body(format!("{{\"uri\": \"{uri}\"}}"))
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(fs::read_to_string("./html/index.html").unwrap())
}


#[get("/script.js")]
async fn script() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(fs::read_to_string("./html/script.js").unwrap())
}

#[get("/style.css")]
async fn style() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(fs::read_to_string("./html/style.css").unwrap())
}


#[get("/{uri}")]
async fn redirect(
    pool: web::Data<Pool>,
    web::Path(uri): web::Path<String>,
) -> impl Responder {

    let con = pool.get().await.unwrap();
    let rows = con.query("select * from links where uri=$1", &[&uri]).await.unwrap();

    if rows.len() >= 1 {
        let redirect: String = rows[0].get("redirect");
        HttpResponse::Found().header("Location", redirect).finish()
    } else {
        HttpResponse::Found().header("Location", "/").finish()
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Error loading .env, does it exist?");
    env::var("LINK_SHORTENER_PASSWORD").expect("LINK_SHORTENER_PASSWORD is not set in the .env");

    let mut cfg = Config::new();
    cfg.host(&env::var("DB_HOST").expect("DB_HOST is not set in the .env"));
    cfg.user(&env::var("DB_USER").expect("DB_USER is not set in the .env"));
    cfg.password(&env::var("DB_PASSWORD").expect("DB_PASSWORD is not set in the .env"));
    cfg.dbname(&env::var("DB_NAME").expect("DB_NAME is not set in the .env"));
    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::new(mgr, 10);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(redirect)
            .service(index)
            .service(script)
            .service(style)
            .service(shorten)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

