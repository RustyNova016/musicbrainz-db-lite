use redirect::RecordingGidRedirect;
use welds::{state::DbState, Client, WeldsError, WeldsModel};

pub mod redirect;

#[derive(Debug, WeldsModel)]
#[welds(table = "recordings")]
#[welds(HasMany(mbid, RecordingGidRedirect, "new_id"))]
pub struct Recording {
    #[welds(primary_key)]
    pub id: i64,

    pub mbid: String,

    pub title: String,

    pub length: Option<i64>,

    pub disambiguation: Option<String>,

    pub annotation: Option<String>,
}

impl Recording {
    pub async fn find_by_mbid(
        client: &dyn Client,
        mbid: &str,
    ) -> Result<Option<DbState<Recording>>, WeldsError> {
        Ok(RecordingGidRedirect::where_col(|c| c.gid.equal(mbid))
            .where_col(|c| c.deleted.equal(0))
            .map_query(|r| r.recording)
            .limit(1)
            .run(client)
            .await?
            .pop())
    }

    pub fn replace(mut row: DbState<Recording>, new: Recording) -> DbState<Self> {
        let id = row.id;

        *row = new;
        row.id = id;

        row
    }
}
