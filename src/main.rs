#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[get("/hello/<name>")]
fn say_hello(name: &str) -> String {
    // IDK why I put this here
    // consider it to be an easter egg

    format!("Hello, {}!", name)
}

#[get("/get_all_events/<user>")]
fn get_all_events(user: &str) -> String {
    // Returns all events in a given user's todo list.

    let file_path = format!(r"src\Data\{}.json", user);     // Opens the json file for the given user.
    let text = fs::read_to_string(&file_path).unwrap();     // Retrieves all the data stored in the JSON file as a string.
    text                                                    // Returns the file contents as a String.
                                                            // The contents will be printed to the screen.
}

#[get("/add_event/<user>/<event>")]
fn add_event(user: &str, event: &str) -> String {
    // Creates new events for a given user.

    let file_path = format!(r"src\Data\{}.json", user);     // Opens the JSON file for the given user.
    let text = fs::read_to_string(&file_path).unwrap();     // Retrieves all teh data stored in the JSON file as a string.

    let mut user_data: User = serde_json::from_str(&text).unwrap();      // Turns JSON into User struct.
    user_data.Todo.push(event.to_string());                              // Adds new event to User struct todo vec.

    let json = serde_json::to_string_pretty(&user_data).unwrap();        // Converts User struct into JSON.
    fs::write(file_path, json).expect("Unable to write to file");        // Writes new JSON to the users JSON file.

    println!("Successfully added '{}' to {}'s todo list", event, user);
    return format!("Successfully added '{}' to {}'s todo list", event, user);
}

#[get("/delete_event/<user>/<event>")]
fn delete_event(user: &str, event: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", user);
    let text = fs::read_to_string(&file_path).unwrap();

    let mut user_data: User = serde_json::from_str(&text).unwrap();

    for list_index in 0..user_data.Todo.len() {

        println!("User Data: {}", user_data.Todo[list_index]);
        if user_data.Todo[list_index] == event {
            user_data.Todo.remove(list_index);

            let json = serde_json::to_string_pretty(&user_data).unwrap();
            fs::write(file_path, json).expect("Unable to write to file");

            println!("Successfully removed '{}' from {}'s todo list", event, user);
            return format!("Successfully removed '{}' from {}'s todo list", event, user);
        }
    }

    println!("The event '{}' doesn't exist inside of {}'s todo list", event, user);
    return format!("The event '{}' doesn't exist inside of {}'s todo list", event, user);
}

// functions to implement
fn edit_event(user: &str, current_event: &str, new_event: &str) {}
fn create_new_user(user: &str) {}
fn delete_user(user: &str) {}
fn help() {}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    Todo: Vec<String>,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hello])
                   .mount("/", routes![get_all_events])
                   .mount("/", routes![add_event])
                   .mount("/", routes![delete_event])
}
