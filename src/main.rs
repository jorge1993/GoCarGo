#[macro_use] 
extern crate rocket;
use rocket::fs::NamedFile;
use std::path::Path;
use std::path::PathBuf;


use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}



#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("html/index.html").await.ok()
}


#[get("/js/<file..>")]
async fn js(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/js/").join(file)).await.ok().map(|mut f| {
        f.set_content_type(rocket::http::ContentType::JavaScript);
        f
    })
}

#[get("/css/<file..>")]
async fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/css/").join(file)).await.ok()
}

#[get("/img/<file..>")]
async fn img(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/img/").join(file)).await.ok()
}

#[get("/config")]
fn config() -> &'static str {
    "Hello, teacher!"
}

#[get("/favicon")]
fn favicon() -> &'static str {
    "Hello, teacher!"
}

#[post("/state")]
fn state() -> &'static str {
    "Hello, Jorge!"
}



#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS)
    .mount("/", routes![index])
    .mount("/config", routes![config])
    .mount("/state", routes![state])
    .mount("/favicon.ico", routes![favicon])
    .mount("/js", routes![js])
    .mount("/css", routes![css])
    .mount("/img", routes![img])
}