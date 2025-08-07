mod auth;
mod handlers;

use crate::auth::validator;
use crate::handlers::{
    images::get_images,
    upload::{update_images, upload_image},
};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type ImageList = Arc<Mutex<Vec<ImageMeta>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageMeta {
    pub path: String,
    pub title: String,
    pub description: String,
}

pub fn load_images() -> Vec<ImageMeta> {
    let path = PathBuf::from("./uploads/images.json");
    if path.exists() {
        let file = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&file).unwrap_or_default()
    } else {
        Vec::new()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let images = Arc::new(Mutex::new(load_images()));

    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validator);

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(images.clone()))
            .service(Files::new("/uploads", "./uploads").show_files_listing())
            .route("/images", web::get().to(get_images))
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .route("/images", web::get().to(get_images))
                    .route("/upload", web::post().to(upload_image))
                    .route("/update", web::post().to(update_images)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

