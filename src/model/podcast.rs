/// Pocketcasts podcast information.
#[derive(Clone, Debug, Deserialize)]
pub struct Podcast {
    pub uuid: String,
    #[serde(rename = "episodesSortOrder")]
    pub episodes_sort_order: i64,
    #[serde(rename = "autoStartFrom")]
    pub auto_start_from: i64,
    pub title: String,
    pub author: String,
    pub description: String,
    pub url: String,
    #[serde(rename = "lastEpisodePublished")]
    pub last_episode_published: String,
    pub unplayed: bool,
    #[serde(rename = "lastEpisodeUuid")]
    pub last_episode_uuid: String,
    #[serde(rename = "lastEpisodePlayingStatus")]
    pub last_episode_playing_status: i64,
    #[serde(rename = "lastEpisodeArchived")]
    pub last_episode_archived: bool,
}

/// User's podcasts.
#[derive(Clone, Debug, Deserialize)]
pub struct UserPodcasts {
    pub podcasts: Vec<Podcast>,
}
