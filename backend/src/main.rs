use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct ImageMeta {
    path: String,
    title: String,
    description: String,
}

type ImageList = Arc<Mutex<Vec<ImageMeta>>>;

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

async fn validator(
    req: actix_web::dev::ServiceRequest,
    credentials: BasicAuth,
) -> Result<actix_web::dev::ServiceRequest, (actix_web::Error, actix_web::dev::ServiceRequest)> {
    let username = credentials.user_id();
    let password = credentials.password().unwrap_or("");

    let allowed = [
        ("admin1", "password123"),
        ("admin2", "s3cr3t"),
        ("admin3", "pa55w0rd"),
    ];

    if allowed
        .iter()
        .any(|(u, p)| *u == username && *p == password)
    {
        Ok(req)
    } else {
        Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
    }
}

fn load_images() -> Vec<ImageMeta> {
    let path = PathBuf::from("./uploads/images.json");
    if path.exists() {
        let file = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&file).unwrap_or_default()
    } else {
        Vec::new()
    }
}

async fn get_images(data: web::Data<ImageList>) -> impl Responder {
    let images = data.lock().unwrap();
    HttpResponse::Ok().json(&*images)
}

async fn upload_image(mut payload: Multipart, data: web::Data<ImageList>) -> impl Responder {
    let mut title = String::new();
    let mut desc = String::new();
    let mut filename = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let name = content_disposition.get_name().unwrap();

        if name == "file" {
            let ext = content_disposition.get_filename().unwrap_or("upload.jpg");
            let ext = PathBuf::from(ext);
            let ext = ext.extension().and_then(|e| e.to_str()).unwrap_or("jpg");

            let upload_dir = PathBuf::from("./uploads/");
            if !upload_dir.exists() {
                fs::create_dir_all(&upload_dir).unwrap();
            }

            let unique_name = format!("{}.{}", Uuid::new_v4(), ext);
            let filepath = upload_dir.join(&unique_name);
            let mut f = fs::File::create(&filepath).unwrap();

            while let Some(chunk) = field.next().await {
                f.write_all(&chunk.unwrap()).unwrap();
            }

            filename = Some(unique_name);
        } else if name == "title" {
            let mut buffer = Vec::new();
            while let Some(chunk) = field.next().await {
                buffer.extend_from_slice(&chunk.unwrap());
            }
            title = String::from_utf8(buffer).unwrap_or_default();
        } else if name == "description" {
            let mut buffer = Vec::new();
            while let Some(chunk) = field.next().await {
                buffer.extend_from_slice(&chunk.unwrap());
            }
            desc = String::from_utf8(buffer).unwrap_or_default();
        }
    }

    if let Some(file) = filename {
        let new_image = ImageMeta {
            path: format!("/uploads/{}", file),
            title,
            description: desc,
        };

        let mut list = data.lock().unwrap();
        list.push(new_image.clone());
        let json = serde_json::to_string_pretty(&*list).unwrap();
        fs::write("./uploads/images.json", json).unwrap();

        HttpResponse::Ok().json(new_image)
    } else {
        HttpResponse::BadRequest().body("Missing file")
    }
}

async fn update_images(
    new_images: web::Json<Vec<ImageMeta>>,
    data: web::Data<ImageList>,
) -> impl Responder {
    let mut list = data.lock().unwrap();
    *list = new_images.into_inner();
    let json = serde_json::to_string_pretty(&*list).unwrap();
    if let Err(e) = std::fs::write("./uploads/images.json", json) {
        eprintln!("Failed to write images.json: {}", e);
        return HttpResponse::InternalServerError().body("Failed to update images");
    }
    HttpResponse::Ok().body("Images updated")
}
