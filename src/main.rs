#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs;

const WEB_ROOT: &str = "root";

mod posts;

use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/raw")]
fn index_markdown() -> Option<NamedFile> {
    NamedFile::open(Path::new(WEB_ROOT).join("index.md")).ok()
}
#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/cv/raw")]
fn cv_markdown() -> Option<NamedFile> {
    NamedFile::open(Path::new(WEB_ROOT).join("cv.md")).ok()
}
#[get("/cv")]
fn cv() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("cv", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, cv, index_markdown, cv_markdown])
        .mount("/css", StaticFiles::from(WEB_ROOT.to_owned() + "/css"))
        .mount("/figs", StaticFiles::from(WEB_ROOT.to_owned() + "/figs"))
        .mount(
            "/scripts",
            StaticFiles::from(WEB_ROOT.to_owned() + "/scripts"),
        )
        .mount(
            "/posts",
            routes![
                posts::index,
                posts::index_markdown,
                posts::post,
                posts::post_markdown
            ],
        )
        .mount(
            "/posts/figs",
            StaticFiles::from(WEB_ROOT.to_owned() + "/posts/figs"),
        )
        .attach(Template::fairing())
        .launch();
}
