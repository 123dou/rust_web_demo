use actix_web::{post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Info {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[post("/user/info")]
pub async fn info(info: web::Json<Info>) -> web::Json<Info> {
    println!("=========={info:?}=========");
    web::Json(info.clone())
}
