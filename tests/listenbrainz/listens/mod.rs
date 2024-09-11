use chrono::DateTime;
use listenbrainz::raw::Client;
use musicbrainz_db_lite::api::listenbrainz::listen_collection::SaveListenPayload;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use welds::TransactStart;

use crate::setup_database;
use crate::setup_file_database;

#[tokio::test]
#[serial_test::serial]
async fn should_insert_dump() {
    let client = setup_database().await.unwrap();

    let lb_client = Client::new();
    let dump = lb_client.user_listens("RustyNova", None, Some(1726041017), Some(100)).unwrap();

    let trans = client.begin().await.unwrap();
    dump.payload.save_listen_payload_in_transaction(&trans, 1726041017, 100).await.unwrap();
    //println!("{:?}", trans);
    assert!(trans.commit().await.is_ok());

    assert_eq!(Listen::all().count(&client).await.unwrap(), 99)
}