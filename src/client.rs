use super::error::PocketcastsError;
use super::model::episode::{Episode, NewReleases};
use super::model::podcast::UserPodcasts;
use super::model::subscription::SubscriptionStatus;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

static API_URL: &str = "https://api.pocketcasts.com";

/// The authenticated pocketcasts client.
#[derive(Debug, Clone)]
pub struct Pocketcasts {
    pub access_token: String,
}

#[derive(Serialize)]
struct V {
    v: i64,
}

#[derive(Serialize)]
struct EpisodeRequest {
    pub uuid: String,
}

impl Pocketcasts {
    fn default_headers(&self) -> HeaderMap {
        let bearer = format!("Bearer {}", self.access_token);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, bearer.parse().unwrap());
        headers
    }

    async fn get(&self, url: &str) -> Result<reqwest::Response, failure::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .headers(self.default_headers())
            .send()
            .await?;
        Ok(response)
    }

    async fn post<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, failure::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .headers(self.default_headers())
            .json(body)
            .send()
            .await?;
        Ok(response)
    }

    /// Create new client with the given access token.
    pub fn new(access_token: String) -> Self {
        Pocketcasts { access_token }
    }

    /// Login using email and password, returning an authenticated client.
    pub async fn login(email: String, password: String) -> Result<Pocketcasts, failure::Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/user/login", &API_URL);
        let response: serde_json::Value = client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "scope": "webplayer"
            }))
            .send()
            .await?
            .json()
            .await?;

        let access_token = response["token"]
            .as_str()
            .ok_or(PocketcastsError::Unauthorized)?
            .to_string();

        Ok(Self::new(access_token))
    }

    /// Get the user's subscription status.
    pub async fn subscription_status(&self) -> Result<SubscriptionStatus, failure::Error> {
        let url = format!("{}/subscription/status", &API_URL);
        let status: SubscriptionStatus = self.get(&url).await?.json().await?;
        Ok(status)
    }

    /// Get the user's subscribed podcasts.
    pub async fn user_podcasts(&self) -> Result<UserPodcasts, failure::Error> {
        let url = format!("{}/user/podcast/list", &API_URL);
        let body = V { v: 1 };
        let podcasts: UserPodcasts = self.post(&url, &body).await?.json().await?;
        Ok(podcasts)
    }

    /// Get the user's new releases.
    pub async fn new_releases(&self) -> Result<NewReleases, failure::Error> {
        let url = format!("{}/user/new_releases", &API_URL);
        let body = V { v: 1 };
        let releases: NewReleases = self.post(&url, &body).await?.json().await?;
        Ok(releases)
    }

    /// Return episode with the specified uuid.
    pub async fn episode(&self, uuid: String) -> Result<Episode, failure::Error> {
        let url = format!("{}/user/episode", &API_URL);
        let body = EpisodeRequest { uuid };
        let episode: Episode = self.post(&url, &body).await?.json().await?;
        Ok(episode)
    }
}
