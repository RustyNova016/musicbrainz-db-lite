use std::collections::HashMap;

use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::models::{
    listenbrainz::msid_mapping::MsidMapping,
    musicbrainz::{recording::Recording, user::User},
};
use crate::utils::sqlx_utils::entity_relations::{JoinCollection, JoinRelation};

use super::Listen;

impl Listen {
    pub async fn get_recording_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Option<Recording>, crate::Error> {
        let user = User::find_by_name(conn, &self.user)
            .await?
            .expect("User should be in due to foreign keys");

        let recording_mbid =
            MsidMapping::find_by_user_msid2(conn, user.id, &self.recording_msid).await?;

        match recording_mbid {
            None => Ok(None),
            Some(mapping) => Recording::get_or_fetch(conn, &mapping.recording_mbid).await,
        }
    }

    /// Takes in a vec of listens, and associate them to their listened recordings. This only need the user's id to determine the mappings
    ///
    /// The recordings must be prefetched
    pub async fn get_recordings_as_batch(
        conn: &mut SqliteConnection,
        user_id: i64,
        listens: Vec<Listen>,
    ) -> Result<HashMap<i64, (Listen, Vec<Recording>)>, crate::Error> {
        let ids = listens.iter().map(|v| v.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Recording>> = sqlx::query_as("
            SELECT
                listens.id as original_id,
                recordings.*
            FROM
                listens
                INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
                INNER JOIN recordings ON recordings_gid_redirect.new_id = recordings.id
            WHERE
                msid_mapping.user = ?
                AND listens.id IN (
                    SELECT value FROM JSON_EACH(?)
                )"
        ).bind(user_id).bind(id_string).fetch_all(conn).await?;

        Ok(JoinCollection::from(joins).into_hashmap(listens, |id, listen| &listen.id == id))
    }

    /// Get the recordings that aren't in the database but have listens among a list of listens
    pub async fn get_unfetched_recordings_ids(
        conn: &mut SqliteConnection,
        user_id: i64,
        listens: &[Listen],
    ) -> Result<Vec<String>, crate::Error> {
        let ids = listens.iter().map(|v| v.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        Ok(
            sqlx::query_scalar!(r#"
                    SELECT DISTINCT
                        recordings_gid_redirect."gid"
                    FROM
                        listens
                        INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                        INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                        INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
                    WHERE
                        recordings_gid_redirect.deleted = 0
                        AND recordings_gid_redirect.new_id IS NULL
                        AND msid_mapping.user = ?
                        AND listens.id IN (SELECT value FROM JSON_EACH(?))
                        "#,
                        user_id, id_string
                )
                .fetch_all(conn)
                .await?)
    }
}
