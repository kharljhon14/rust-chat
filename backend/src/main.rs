use routes::chat::chat;

mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![chat])
        .launch()
        .await;
}
