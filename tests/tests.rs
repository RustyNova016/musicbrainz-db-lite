use std::fs;
use std::fs::File;

use musicbrainz_db_lite::database::tables::create_database;
use musicbrainz_db_lite::utils::check_db_integrity;
use welds::connections::sqlite::SqliteClient;
use welds::WeldsError;

mod listenbrainz;

/// Connect and setup a DB to test on
pub async fn setup_database() -> Result<SqliteClient, WeldsError> {
    let client = welds::connections::sqlite::connect("sqlite::memory:").await?;
    create_database(&client).await?;
    Ok(client)
}

/// Connect and setup a DB to test on. Use this if you actually need to see values for debugging
pub async fn setup_file_database() -> Result<SqliteClient, WeldsError> {
    if std::fs::exists("./tests/test_db.db").unwrap() {
        fs::remove_file("./tests/test_db.db").unwrap();
    }

    File::create_new("./tests/test_db.db").unwrap();
    let client = welds::connections::sqlite::connect("sqlite:./tests/test_db.db").await?;
    create_database(&client).await?;
    Ok(client)
}

#[tokio::test]
#[serial_test::serial]
async fn should_setup_database() {
    let res = setup_database().await;

    assert!(res.is_ok())
}

#[tokio::test]
#[serial_test::serial]
async fn model_should_match_db() {
    let client = setup_database().await.unwrap();

    assert!(check_db_integrity(&client).await.is_ok_and(|v| v))
}