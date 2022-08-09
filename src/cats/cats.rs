use std::path::{Path, PathBuf};
use rocket::http::{ContentType, Header, Status};
use rocket::response::{content, status};
use rocket::serde::{Serialize, Deserialize};

#[get("/cats")]
pub fn cats1() -> status::Accepted<String> {
    status::Accepted(Some(format!("hello cats")))
}

#[get("/cats2")]
pub fn cats2() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::Ok, content::RawJson("{ \"hi\": \"cats2\" }"))
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct RawTeapotJson(&'static str);

#[get("/cats3")]
pub fn cats3() -> RawTeapotJson {
    RawTeapotJson("{ \"hi\": \"cats3\" }")
}

#[get("/cats4")]
pub fn cats4() -> Status {
    Status::NotAcceptable
}

// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// struct MyResponder {
//     // inner: OtherResponder,
//     header: ContentType,
//     more: Header<'static>,
//     // #[response(ignore)]
//     // unrelated: MyType,
// }

#[get("/cats5")]
pub fn cats5() -> Status {
    Status::NotAcceptable
}
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

// 使用 Option，错误时返回错误信息
#[get("/cats6/<file..>")]
pub async fn cats6(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// 使用 Result 返回错误信息
#[get("/cats7/<file..>")]
pub async fn cats7(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    id: u16,
    message: String
}

#[get("/cats8")]
pub fn cats8() -> Json<Task> {
    Json(Task {
        id: 0,
        message: "cats8 的 id 为 0".to_string()
    })
}

#[get("/cats9")]
pub fn cats9() -> Value {
    json!({ "status": "ok", "id": "9", "name": "cats9" })
}
