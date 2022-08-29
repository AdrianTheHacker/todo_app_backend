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
    user_data.todo.push(event.to_string());                              // Adds new event to User struct todo vec.

    let json = serde_json::to_string_pretty(&user_data).unwrap();        // Converts User struct into JSON.
    fs::write(file_path, json).expect("Unable to write to file");        // Writes new JSON to the users JSON file.

    println!("Successfully added '{}' to {}'s todo list", event, user);
    return format!("Successfully added '{}' to {}'s todo list", event, user);
}

#[get("/delete_event/<user>/<event>")]
fn delete_event(user: &str, event: &str) -> String {
    // Deletes the first instance of an event inside of a given user's JSON file.

    let file_path = format!(r"src\Data\{}.json", user);     // Finds the the JSON file that stores a given users data.
    let text = fs::read_to_string(&file_path).unwrap();     // Grabs all the contents of the file.

    let mut user_data: User = serde_json::from_str(&text).unwrap();     // Creating a User struct out of the JSON data
                                                                        // Allows us to modify it easier.

    let mut completion_message = format!("The event '{}' doesn't exist inside of {}'s todo list", event, user);      // Message displayed after completiion.
                                                                                                                    // This is meant for debugging.

    for list_index in 0..user_data.todo.len() {     // Searches for the event, and then deletes it.

        println!("User Data: {}", user_data.todo[list_index]);
        if user_data.todo[list_index] == event {
            user_data.todo.remove(list_index);      // Removes the event from the list

            let json = serde_json::to_string_pretty(&user_data).unwrap();       // Creates JSON data out of the user_data Struct.
            fs::write(file_path, json).expect("Unable to write to file");       // Writes this JSON data back to the user's JSON file.

            completion_message = format!("Successfully removed '{}' from {}'s todo list", event, user);     // Changes the completion_message (for debugging purposes)
            break;
        }
    }

    println!("{}", completion_message);
    return completion_message;
}

#[get("/edit_event/<user>/<current_event>/<new_event>")]
fn edit_event(user: &str, current_event: &str, new_event: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", user);
    let text = fs::read_to_string(&file_path).unwrap();

    let mut user_data: User = serde_json::from_str(&text).unwrap();

    let mut completion_message = format!("The event '{}' doesn't exist inside of {}'s todo list", current_event, user);

    for list_index in 0..user_data.todo.len() {

        println!("User Data: {}", user_data.todo[list_index]);
        if user_data.todo[list_index] == current_event {
            user_data.todo[list_index] = String::from(new_event);

            let json = serde_json::to_string_pretty(&user_data).unwrap();
            fs::write(file_path, json).expect("Unable to write to file");

            completion_message = format!("Successfully edited '{}' to '{}' in {}'s todo list", current_event, new_event, user);
            break;
        }
    }

    println!("{}", completion_message);
    return completion_message;
}

#[get("/create_new_user/<user>")]
fn create_new_user(user: &str) -> String {
    let file_path = format!(r"src\Data\{}.json", user);
    let json = serde_json::to_string_pretty(&User {todo: Vec::new()}).unwrap();
    let completion_message = format!("Sucessfully created {}'s todo list", user);

    fs::write(file_path, json).expect("Unable to create new file / write to that file");

    println!("{}", completion_message);
    return completion_message;
}

// #[get("/delete_user/<user>")]
// fn delete_user(user: &str) -> String {
//     let file_path = format!(r"scr\Data\{}.json", user);
//     let completion_message = format!("Successfully deleted {}'s todo list", user);

//     fs::remove_file(file_path);

//     println!("{}", completion_message);
//     return completion_message;
// }

// functions to implement
// fn help() {}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    todo: Vec<String>,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hello])
                   .mount("/", routes![get_all_events])
                   .mount("/", routes![add_event])
                   .mount("/", routes![delete_event])
                   .mount("/", routes![edit_event])
                   .mount("/", routes![create_new_user])
                   .mount("/", routes![delete_user])
}
