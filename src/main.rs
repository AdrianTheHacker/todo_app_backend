#[macro_use] extern crate rocket;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::fs::File;
use std::io::Read;

#[warn(dead_code)]
#[get("/hello/<name>")]
fn say_hi(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/get_todo_list/<user>")]
fn get_todo_list(user: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", user);
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    format!("{}", json.find_path(&["Todo"]).unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![find_data_value])
                   .mount("/", routes![get_todo_list])
}
