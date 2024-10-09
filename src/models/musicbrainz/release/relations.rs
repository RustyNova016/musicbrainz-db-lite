use sqlx::SqliteConnection;

use super::{LabelInfo, Media, Release};

impl Release {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_medias_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Media>, crate::Error> {
        // First, make sure all the data of the entity is in the database
        let id = self.get_or_fetch_as_complete(conn).await?.id;

        // Next, get all the children
        Ok(sqlx::query_as(
            r#"SELECT
                    medias.*
                FROM
                    releases
                    INNER JOIN medias ON releases.id = medias.release
                WHERE
                    releases.id = ?"#,
        )
        .bind(id)
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_label_infos_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<LabelInfo>, crate::Error> {
        // First, make sure all the data of the entity is in the database
        let id = self.get_or_fetch_as_complete(conn).await?.id;

        // Next, get all the children
        Ok(sqlx::query_as!(
            LabelInfo,
            "SELECT
                    label_infos.*
                FROM 
                    releases
                    INNER JOIN label_infos ON releases.id = label_infos.release
                WHERE
                    releases.id = ?",
            id
        )
        .fetch_all(conn)
        .await?)
    }
}

impl Media {}
