use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]

pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}