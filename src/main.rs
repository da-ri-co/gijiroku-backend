use rocket::{get, launch, routes};

#[get("/hc")]
fn health_check() -> &'static str {
    "Health OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}