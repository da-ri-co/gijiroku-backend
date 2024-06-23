#[macro_use] extern crate rocket;

use std::fs;
use std::io::Read;
use std::path::Path;
use hex::encode;
use rocket::serde::json::Json;
use rocket_cors::CorsOptions;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ディスカッションのリストを表す構造体
#[derive(Serialize, Deserialize, Debug)]
struct DiscussAbstract {
    id: u64, // SQL上のID
    guild_id: u64,
    start_time: u64,
    title: String,
    abst: String,
}

// あるディスカッションのログの全てを表す構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DiscussDetails {
    id: u64, // SQL上のID
    discussion_id: u64,
    speaker_name: String,
    content: String
}

#[get("/all")]
fn get_all_discussions() -> Json<Vec<DiscussAbstract>> {
    let mut discussions = Vec::new();
    let path = Path::new("db/abstract");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(mut file) = fs::File::open(entry.path()) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        if let Ok(discussion) = serde_json::from_str::<DiscussAbstract>(&contents) {
                            discussions.push(discussion);
                        }
                    }
                }
            }
        }
    }

    Json(discussions)

}

#[get("/<id>")]
fn get_discussion_log_by_id(id: String) -> Json<Vec<DiscussDetails>> {
    let mut discussions: Vec<DiscussDetails> = Vec::new();
    let path = Path::new("db/details");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(mut file) = fs::File::open(entry.path()) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        if let Ok(discussion) = serde_json::from_str::<DiscussDetails>(&contents) {
                            discussions.push(discussion);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", discussions);

    Json(discussions.iter().filter_map(|d| { if d.discussion_id.to_string() == id { Some(d.clone()) } else { None} }).collect())


}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/details", data = "<details>")]
async fn get_discussion_details(details: Json<Vec<DiscussDetails>>) -> &'static str {
    for x in details.iter() {
        if let Err(e) = save_json_to_file(&*x, "db/details").await {
            eprintln!("Failed to save discussion details: {}", e);
            return "Failed to save discussion details";
        }
    }
    return "Discussion details saved successfully";
}

#[post("/abstract", data = "<abst>")]
async fn get_discussion_abstract(abst: Json<DiscussAbstract>) -> &'static str {
    if let Err(e) = save_json_to_file(&*abst, "db/abstract").await {
        eprintln!("Failed to save discussion abstract: {}", e);
        return "Failed to save discussion abstract";
    }
    "Discussion abstract saved successfully"
}

async fn save_json_to_file<T: serde::Serialize>(data: &T, directory: &str) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(data).expect("Failed to serialize JSON");
    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash = encode(hasher.finalize());

    let file_path = Path::new(directory).join(format!("{}.json", hash));
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(file_path, json_string)?;
    Ok(())
}




#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/data", routes![get_discussion_details, get_discussion_abstract])
        .mount("/hello", routes![index])
        .mount("/discussions", routes![get_all_discussions, get_discussion_log_by_id])
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch()
        .await?;
    Ok(())
}