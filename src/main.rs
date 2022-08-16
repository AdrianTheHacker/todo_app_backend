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

#[get("/find_data/<name>/<data_key>")]
fn find_data_value(name: &str, data_key: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", name);
    println!("{}", file_path);
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    format!("{}", json.find_path(&[data_key]).unwrap())
}

fn get_todo_list(user: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", name);
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut date).unwrap();

    let json = Json::from_str(&data).unwrap();
    format!("{}", json.find_path(&[data_key]).unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![find_data_value])
}
