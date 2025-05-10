mod encode;
use dotenv::dotenv;
use std::env;
use std::path::Path;
// use  encode::encode::test;
fn main() {
    dotenv().ok();
    let arg: Vec<String> = env::args().collect();

    if arg.len() > 2 {
        if arg[1] == "path" {
            let path_string = Path::new(&arg[2]);
            if path_string.exists() && path_string.is_file() {}
        }
    }

    let port = env::var("GET_PORT").expect("GET_PORT is not in .env");
    println!("PORT {}", port);
    encode::encode::test();
}
