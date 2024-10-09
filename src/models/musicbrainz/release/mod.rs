pub mod relations;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::FromRow;

use crate::utils::macros::{artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, Default, Clone, FromRow, Upsert, MainEntity)]
#[database(table="releases", primary_key= "id", ignore_insert_keys(id), ignore_update_keys(id, mbid))]
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
    pub full_update_date: Option<i64>,

    pub artist_credit: Option<i64>,
}

impl_redirections!(Release, "releases");
impl_artist_credits!(Release, "releases");
impl_get_and_fetch!(Release);

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(table="medias", primary_key= "id", ignore_insert_keys(id), ignore_update_keys(id))]
pub struct Media {
    pub id: i64,
    pub track_count: i64,
    pub title: Option<String>,
    pub position: Option<i64>,
    pub disc_count: Option<i64>,
    pub format: Option<String>,
    pub track_offset: Option<i64>,

    pub release: i64
}

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(table="tracks", primary_key= "id", ignore_insert_keys(id), ignore_update_keys(id, gid))]
pub struct Track {
    pub id: i64,
    pub gid: String,
    pub title: String,
    pub number: String,
    pub length: Option<i64>,
    pub position: i64,

    pub media: i64,
    pub recording: Option<String>,
    pub artist_credit: Option<i64>
}

impl_artist_credits!(Track, "tracks");

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(table="label_infos", primary_key= "id", ignore_insert_keys(id), ignore_update_keys(id, gid))]
pub struct LabelInfo {
    pub id: i64,
    pub catalog_number: Option<String>,
    pub label: String,
    pub release: i64
}