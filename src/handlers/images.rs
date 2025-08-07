use crate::ImageMeta;
use actix_web::{HttpResponse, Responder, web};
use std::sync::{Arc, Mutex};

pub type ImageList = Arc<Mutex<Vec<ImageMeta>>>;

pub async fn get_images(data: web::Data<ImageList>) -> impl Responder {
    let images = data.lock().unwrap();
    HttpResponse::Ok().json(&*images)
}
