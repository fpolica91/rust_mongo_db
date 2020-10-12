use crate::user_service::User;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/get-all-users")]
async fn get_all_users(app_data: web::Data<crate::AppState>) -> impl Responder {
  let action = app_data.service_manager.user.get();
  let result = web::block(move || action).await;
  match result {
    // similar to response.json(result)
    Ok(result) => HttpResponse::Ok().json(result),
    // below is a catch block
    Err(e) => {
      println!("Error while getting, {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[post("/add-user")]
async fn add_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
  let action = app_data.service_manager.user.create(&user);
  let result = web::block(move || action).await;
  match result {
    Ok(result) => HttpResponse::Ok().json(result.inserted_id),
    Err(e) => {
      println!("Error while getting, {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(add_user);
  cfg.service(get_all_users);
}
