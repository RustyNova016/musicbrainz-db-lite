use musicbrainz_db_lite_macros::Upsert;
use sqlx::FromRow;

use crate::utils::macros::{artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(name="releases", null_fields(id), ignore_update_keys(id, mbid))]
pub struct Release {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub date: Option<i64>,
    pub country: Option<String>,
    pub quality: Option<String>,
    pub status: Option<String>,
    pub barcode: Option<String>,
    pub disambiguation: Option<String>,
    pub packaging: Option<String>,
    pub annotation: Option<String>,
    pub artist_credit: Option<i64>,
}

impl_redirections!(Release, "releases");
impl_artist_credits!(Release, "releases");
impl_get_and_fetch!(Release);

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(name="medias", null_fields(id), ignore_update_keys(id))]
pub struct Media {
    pub id: i64,
    pub title: Option<String>,
    pub position: Option<i64>,
    pub disc_count: Option<i64>,
    pub format: Option<String>,

    pub release: i64
}

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(name="tracks", null_fields(id), ignore_update_keys(id, gid))]
pub struct Track {
    pub id: i64,
    pub gid: String,
    pub title: String,
    pub number: String,
    pub position: i64,

    pub media: i64,
    pub recording: Option<String>
}