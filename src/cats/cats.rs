pub fn new() {

}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
