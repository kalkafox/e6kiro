use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct E621Posts {
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
    pub locked_tags: Vec<Value>,
    pub change_seq: i64,
    pub flags: Flags,
    pub rating: Rating,
    pub fav_count: i64,
    pub sources: Vec<String>,
    pub pools: Vec<i64>,
    pub relationships: Relationships,
    pub approver_id: Option<i64>,
    pub uploader_id: i64,
    pub description: String,
    pub comment_count: i64,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub width: i64,
    pub height: i64,
    pub ext: Ext,
    pub size: i64,
    pub md5: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ext {
    Gif,
    Jpg,
    Png,
    Swf,
    Webm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockedTag {
    #[serde(rename = "conditional_dnp")]
    ConditionalDnp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview {
    pub width: i64,
    pub height: i64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rating {
    E,
    Q,
    S,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationships {
    pub parent_id: Option<i64>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub has: bool,
    pub height: i64,
    pub width: i64,
    pub url: String,
    pub alternates: Alternates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alternates {
    #[serde(rename = "720p")]
    pub the_720_p: Option<The0_P>,
    #[serde(rename = "480p")]
    pub the_480_p: Option<The0_P>,
    pub original: Option<Original>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Original {
    #[serde(rename = "type")]
    pub original_type: String,
    pub height: i64,
    pub width: i64,
    pub urls: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct The0_P {
    #[serde(rename = "type")]
    pub the_0__p_type: String,
    pub height: i64,
    pub width: i64,
    pub urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub up: i64,
    pub down: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub general: Vec<String>,
    pub artist: Vec<String>,
    pub copyright: Vec<String>,
    pub character: Vec<String>,
    pub species: Vec<String>,
    pub invalid: Vec<String>,
    pub meta: Vec<String>,
    pub lore: Vec<String>,
}
