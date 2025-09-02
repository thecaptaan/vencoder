use rocket::serde::json::Json;
use crate::controllers::index_controller;
use crate::models::response::SuccessResponse;

#[get("/health")]
pub fn get_user_route() -> Json<SuccessResponse> {
    Json(index_controller::hello_index())
}