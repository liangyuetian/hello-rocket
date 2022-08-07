#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world1!"
}

#[get("/word")]
fn word2() -> &'static str {
    "world!"
}
// #[get("/hello/<name>")]
// fn hello(name: & str) -> String {
//     let mut s1 = "world!".to_string();
//     let s2 = name.to_string();
//     s1.push_str(s2.as_str());
//     format!("Hello, {}!", s1)
// }

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/foo/<_>/bar", rank = 1)] // rank 设置排名
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![hello])
        .mount("/", routes![foo_bar])
        .mount("/", routes![everything])
        .launch()
        .await?;

    Ok(())
}