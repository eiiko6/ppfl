use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use futures_util::{StreamExt, TryStreamExt};
use std::{fs, io::Write, path::PathBuf};
use uuid::Uuid;

use crate::{ImageList, ImageMeta};

pub async fn upload_image(mut payload: Multipart, data: web::Data<ImageList>) -> impl Responder {
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

pub async fn update_images(
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
