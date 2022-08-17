#[macro_use] extern crate rocket;

#[get("/hello/<name>")]
fn say_hi(name: &str) -> String {
    // IDK why I put this here
    // consider it to be an easter egg

    format!("Hello, {}!", name)
}

#[get("/get_todo_list/<user>")]
fn get_todo_list(user: &str) -> String {
    // Gets the todo list of a given user.

    // Throws an error that the file isn't found 
    // if the user doesn't have a todo list.

    let file_path = format!(r"src\Data\{}.json", user);         // Opens the json file for the given user.
    let text = std::fs::read_to_string(&file_path).unwrap();    // Turns contents of the file into a String.
    text                                                        // Returns the file contents as a String.
                                                                // The contents will be printed to the screen.
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hi])
                   .mount("/", routes![get_todo_list])
}
