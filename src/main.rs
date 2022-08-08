#[macro_use] extern crate rocket;

mod cats;
mod dogs;

use std::io;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![dogs::dogs::index])
        .mount("/", routes![dogs::dogs::hello])
        .mount("/", routes![dogs::dogs::word])
        .mount("/", routes![dogs::dogs::foo_bar])
        .mount("/", routes![dogs::dogs::everything])
        .mount("/", routes![dogs::dogs::delay])
        .mount("/", routes![dogs::dogs::blocking_task])
        .mount("/", routes![dogs::dogs::admin_panel_redirect])
        .mount("/", routes![dogs::dogs::login])
        .mount("/", routes![dogs::dogs::logout])
        .mount("/", routes![dogs::dogs::user])
        .launch()
        .await?;

    Ok(())
}