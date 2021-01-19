use serde::Deserialize;

#[derive(Debug)]
pub enum DataType {
    URL (String),
    File (String)
}

#[derive(Deserialize, Debug)]
pub struct Me {
    pub user_id: Option<i32>,
    pub email: Option<String>,
    pub limit: i32,
    pub limit_ttl: i32,
    pub quota: i32,
    pub quota_ttl: i32,
    pub user_limit: i32,
    pub user_limit_ttl: i32,
    pub user_quota: i32,
    pub user_quota_ttl: i32
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub from: f32,
    pub to: f32,
    pub anilist_id: i32,
    pub at: f32,
    pub season: String,
    pub anime: String,
    pub filename: String,
    pub episode: i32,
    pub tokenthumb: String,
    pub similarity: f32,
    pub title: String,
    pub title_native: String,
    pub title_english: String,
    pub title_chinese: String,
    pub title_romaji: String,
    pub mal_id: i32,
    pub synonyms: Vec<String>,
    pub synonyms_chinese: Vec<String>,
    pub is_adult: bool
}

#[derive(Deserialize, Debug)]
pub struct Search {
    #[serde(rename = "RawDocsCount")]
    pub raw_docs_count: i64,
    #[serde(rename = "RawDocsSearchTime")]
    pub raw_docs_search_time: i64,
    #[serde(rename = "ReRankSearchTime")]
    pub re_rank_search_time: i64,
    #[serde(rename = "CacheHit")]
    pub cache_hit: bool,
    pub docs: Vec<SearchResult>,
    pub limit: i32,
    pub limit_ttl: i32,
    pub quota: i32,
    pub quota_ttl: i32
}
