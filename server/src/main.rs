#[macro_use]
extern crate rocket;

use rocket::Request;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
struct User
{
    first_name: String,
    last_name: String,
    email: String,
    weight: u8
}

#[catch(404)]
fn not_found(req: &Request) -> String
{
    return String::from("Not found");
}

#[get("/data/<pretty>")]
fn get_user(pretty: bool) -> String
{
    let user = User {
        first_name: String::from("Alfred"),
        last_name: String::from("Rydahl"),
        email: String::from("a.rottger_rydahl@live.dk"),
        weight: 80
    };

    if pretty { 
        return serde_json::to_string_pretty(&user)
        .unwrap_or(format!("{{ status: Error }}")); 
    }
    else { 
        return serde_json::to_string(&user)
        .unwrap_or(format!("{{ status: Error }}")); 
    }
}

#[get("/")]
fn index() -> &'static str 
{
    return "Hello World!";
}

#[get("/<name>")]
fn index_name(name: &str) -> String 
{
    return format!("Hello {}!", name);
}

#[launch]
fn rocket() -> _ 
{
    let rockets = rocket::build()
        .mount("/", 
        routes![
            index, 
            index_name,
            get_user
            ]
        )
        .register("/", catchers![not_found]);

    rockets
}