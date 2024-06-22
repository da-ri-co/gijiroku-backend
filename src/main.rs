#[macro_use] extern crate rocket;

use rocket_cors::CorsOptions;
use serde::{Deserialize, Serialize};

// ディスカッションのリストを表す構造体
#[derive(Serialize, Deserialize, Debug)]
struct DiscussAbstract {
    id: String, // SQL上のID
    guild_id: String,
    start_time: u64,
    title: String,
    abst: String,
}

// あるディスカッションのログの全てを表す構造体
#[derive(Serialize, Deserialize, Debug)]
struct DiscussLog {
    
}

#[get("/all")]
fn get_all_discussions() -> String {
    let mock: Vec<DiscussAbstract> =  vec![
        DiscussAbstract {
            id: "1".to_string(),
            guild_id: "1".to_string(),
            start_time: 1630000000,
            title: "test1".to_string(),
            abst: "test1".to_string(),
        },
        DiscussAbstract {
            id: "2".to_string(),
            guild_id: "2".to_string(),
            start_time: 1630005000,
            title: "test2".to_string(),
            abst: "test2".to_string(),
        },
    ];

    serde_json::to_string(&mock).unwrap()
}

#[get("/<id>")]
fn get_discussion_log_by_id(id: String) -> String {

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .mount("/discussions", routes![get_all_discussions])
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch()
        .await?;
    Ok(())
}