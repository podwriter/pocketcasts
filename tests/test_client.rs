
extern crate dotenv;
extern crate once_cell;
extern crate pocketcasts;
extern crate tokio_test;

use once_cell::sync::OnceCell;
use pocketcasts::Pocketcasts;
use std::env;
use std::sync::Mutex;

struct ClientCredentials {
    pub email: String,
    pub password: String,
}

impl ClientCredentials {
    pub fn default() -> ClientCredentials {
        dotenv::dotenv().ok();
        let email = env::var("PC_EMAIL").expect("Missing env var PC_EMAIL");
        let password = env::var("PC_PASSWORD").expect("Missing env var PC_PASSWORD");
        ClientCredentials { email, password }
    }
}

static POCKETCASTS: OnceCell<Mutex<Pocketcasts>> = OnceCell::new();

fn login() -> Mutex<Pocketcasts> {
    tokio_test::block_on(async {
        let credentials = ClientCredentials::default();
        let pc = Pocketcasts::login(credentials.email, credentials.password)
            .await
            .unwrap();
        Mutex::new(pc)
    })
}

fn pocketcasts_client() -> Pocketcasts {
    let pc = POCKETCASTS.get_or_init(login);
    pc.lock().unwrap().clone()
}

#[test]
fn test_subscription_status() {
    let pc = pocketcasts_client();
    tokio_test::block_on(async {
        let status = pc.subscription_status().await;
        assert!(status.is_ok());
    });
}

#[test]
fn test_user_podcasts() {
    let pc = pocketcasts_client();
    tokio_test::block_on(async {
        let podcasts = pc.user_podcasts().await;
        assert!(podcasts.is_ok());
    });
}

#[test]
fn test_new_releases() {
    let pc = pocketcasts_client();
    tokio_test::block_on(async {
        let releases = pc.new_releases().await;
        assert!(releases.is_ok());
    });
}

#[test]
fn test_episode() {
    let pc = pocketcasts_client();
    tokio_test::block_on(async {
        let releases = pc.new_releases().await.unwrap();
        let episode_uuid = releases.episodes[0].uuid.to_string();
        let episode = pc.episode(episode_uuid.to_string()).await;
        assert!(episode.is_ok());
    });
}
