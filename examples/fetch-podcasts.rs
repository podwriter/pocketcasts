use pocketcasts::Pocketcasts;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let email = env::var("PC_EMAIL").expect("Missing env var PC_EMAIL");
    let password = env::var("PC_PASSWORD").expect("Missing env var PC_PASSWORD");

    let pc = Pocketcasts::login(email, password)
        .await
        .expect("Failed to login");

    let podcasts = pc
        .user_podcasts()
        .await
        .expect("Failed to get user podcasts");

    dbg!(podcasts);
}
