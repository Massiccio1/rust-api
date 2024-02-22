#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use serde_json::json;

mod my_random;

// Try visiting:
//   http://127.0.0.1:8060/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

//  /random
#[get("/")]
fn random_id() -> String {
    format!("{}", my_random::get_random_int(100000))
}

//  /random/<max>
//  /random/100
#[get("/<max>")]
fn random_range(max: u32) -> String {
    format!("{}", my_random::get_random_int(max))
}

//  /random/bool
#[get("/bool")]
fn random_bool() -> String {
    format!("{}", my_random::get_random_bool())
}

#[get("/")]
fn status() -> String {
    format!("service is up")
}

//      curl -X POST -H 'Content-Type: application/json' --data '{"key1": "1", "key2": "key string 2"}' http://127.0.0.1:8060/post

#[post("/", format = "json", data = "<body>")]
fn body(body: &str) -> String {
    let json: HashMap<String, String> = serde_json::from_str(body).unwrap();
    let mut res = format!("{:?}", json);
    if json.contains_key("key1") {
        res = res + "\n" + "key1 val: " + &json["key1"];
    }
    if json.contains_key("key2") {
        res = res + "\n" + "key2 val: " + &json["key2"];
    }
    format!("{}", res)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8060)))
        .mount("/", routes![status])
        .mount("/hello", routes![world])
        .mount("/random", routes![random_id, random_range, random_bool])
        .mount("/post", routes![body])
}
