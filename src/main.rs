#[macro_use] extern crate rocket;

mod cats;
mod dogs;

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
        .mount("/", routes![dogs::dogs::user_id])
        .mount("/", routes![dogs::dogs::login])
        .mount("/", routes![dogs::dogs::logout])
        .mount("/", routes![dogs::dogs::user])
        .mount("/", routes![cats::cats::cats1])
        .mount("/", routes![cats::cats::cats2])
        .mount("/", routes![cats::cats::cats3])
        .mount("/", routes![cats::cats::cats4])
        .mount("/", routes![cats::cats::cats5])
        .mount("/", routes![cats::cats::cats6])
        .mount("/", routes![cats::cats::cats7])
        .mount("/", routes![cats::cats::cats8])
        .mount("/", routes![cats::cats::cats9])
        .launch()
        .await?;

    Ok(())
}