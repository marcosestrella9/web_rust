#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::response::content;
use rocket::response::content::Html;
use rocket_upload::MultipartDatas;
use rocket::http::ContentType;
use std::path::{Path, PathBuf};
use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(r#"
        <title>CSV to Postgresql</title>
        <h1>Seleccione archivo csv </h1>
        <form action="/conv" method="get">
            <input type="file" id="myFile" name="filename">
            <input type="submit">
        </form>
    "#)
}

#[get("/conv?<filename>")]
fn new(filename:String) -> String{
    format!("Archivo subido correctamente {} ", filename.to_string())
    
}

fn main() {
    rocket::ignite().mount("/", routes![index,new]).launch();
}


