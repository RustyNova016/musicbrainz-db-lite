use musicbrainz_rs_nova::entity::artist_credit::ArtistCredit as MBArtistCredit;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::{
    artist::Artist,
    artist_credit::{ArtistCredit, ArtistCredits},
};

impl ArtistCredits {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: Vec<MBArtistCredit>,
    ) -> Result<ArtistCredits, sqlx::Error> {
        let mut rows = Vec::new();
        let mut position = 0;

        for resp in value {
            rows.push(ArtistCredit {
                artist_credit: Default::default(),
                artist_gid: resp.artist.id.clone(),
                join_phrase: resp.joinphrase.unwrap_or("".to_string()),
                name: resp.name,
                position: position,
            });

            Artist::save_api_response_recursive(conn, resp.artist).await?;

            position += 1;
        }

        ArtistCredits::save(conn, &rows).await
    }
}
