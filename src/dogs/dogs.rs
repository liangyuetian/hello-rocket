use std::io;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/word")]
pub fn word() -> &'static str {
    "world!"
}

#[get("/hello/<name>/<age>/<cool>")]
pub fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/foo/<_>/bar", rank = 1)] // rank 设置排名
pub fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")]
pub fn everything() -> &'static str {
    "Hey, you're here."
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

// 重定向
#[get("/admin", rank = 3)]
pub fn admin_panel_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

/// Retrieve the user's ID, if any.
#[get("/user_id")]
pub fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

#[get("/login")]
pub fn login() -> String {
    String::from("login page")
}
// 重定向
/// Remove the `user_id` cookie.
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

// 接受比较复杂的参数
#[derive(FromForm)]
pub struct User<'r> {
    name: &'r str,
    active: bool,
}

#[get("/?hello&<id>&<user..>")]
pub fn user(id: usize, user: User<'_>) {
    // hello&\
    // name=Bob+Smith&\
    // id=1337&\
    // active=yes\
    assert_eq!(id, 1337);
    assert_eq!(user.name, "Bob Smith");
    assert_eq!(user.active, true);
}