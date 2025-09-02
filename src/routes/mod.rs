pub mod public_routes;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        public_routes::get_user_route
    ]
}
