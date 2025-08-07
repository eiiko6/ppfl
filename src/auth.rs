use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
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
