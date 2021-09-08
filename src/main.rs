#[macro_use]
extern crate rocket;

use std::env;
use rocket::fs::relative;
use rocket::fs::FileServer;

#[catch(404)]
fn not_found() -> &'static str
{
    return "Not found";
}

#[get("/<name>")]
fn get_name(name: &str) -> String 
{
    return format!("Hello {}!", name);
}

#[launch]
fn rocket_launch() -> _
{
    // Get the port number from Heroku
    //let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    let roc = rocket::build()
        .mount("/data", routes![get_name])
        .register("/data", catchers![not_found]);

    roc
}