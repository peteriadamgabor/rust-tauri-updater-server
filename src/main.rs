#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType};
use std::{fs, path::Path};

#[get("/<app>/<target>/<current_version>")]
fn get_app_version(app: &str, target: &str, current_version: &str) -> (Status, (ContentType, String)) {

    let version: String = fs::read_to_string(Path::new(&format!("{}/apps/{}/version.txt", env!("CARGO_MANIFEST_DIR"), app))).expect("version file is not exist");
    if version != current_version.to_string() {
        let updater_jsonn: String = fs::read_to_string(Path::new(&format!("{}/apps/{}/{}/{}/release.json", env!("CARGO_MANIFEST_DIR"), app, version, target))).expect("version file is not exist");
        (Status::Ok, (ContentType::JSON, updater_jsonn))
    }
    else {
        (Status::NoContent, (ContentType::JSON,  String::from(" ")))
    }
}

#[get("/health")]
fn health() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON,  "{ \"status\": 200 }"))
}

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (Status::NoContent, (ContentType::JSON,  "{ \"status\": 204, \"content\": \"No content this site.\" }"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health, get_app_version])
}