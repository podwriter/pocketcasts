/// Pocketcasts episode information.
#[derive(Clone, Debug, Deserialize)]
pub struct Episode {
    pub uuid: String,
    pub url: String,
    pub published: String,
    pub duration: i64,
    #[serde(rename = "fileType")]
    pub file_type: String,
    pub title: String,
    pub size: String,
    #[serde(rename = "playingStatus")]
    pub playing_status: i64,
    #[serde(rename = "playedUpTo")]
    pub played_up_to: i64,
    pub starred: bool,
    #[serde(rename = "podcastUuid")]
    pub podcast_uuid: String,
    #[serde(rename = "podcastTitle")]
    pub podcast_title: String,
    #[serde(rename = "episodeType")]
    pub episode_type: String,
    #[serde(rename = "episodeSeason")]
    pub episode_season: i64,
    #[serde(rename = "episodeNumber")]
    pub episode_number: i64,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewReleases {
    pub total: i64,
    pub episodes: Vec<Episode>,
}
