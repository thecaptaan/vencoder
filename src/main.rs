#[macro_use] extern crate rocket;

mod models;
mod controllers;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::get_routes())
}
