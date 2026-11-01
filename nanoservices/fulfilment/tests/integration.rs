//! Integration tests on the testkit: a database per test with every
//! migration applied. Skipped without TEST_DATABASE_URL. The `#[ignore]`d
//! ones are templates to un-ignore as the pieces land.

use basable_testkit::TestDb;

#[tokio::test]
async fn migrations_apply_and_schema_exists() {
    let Some(db) = TestDb::from_env().await else { return };
    let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_namespace WHERE nspname = 'nano_fulfilment')")
        .fetch_one(db.migrator())
        .await
        .unwrap();
    assert!(exists, "schema nano_fulfilment must exist after the migrations");
}

#[tokio::test]
#[ignore = "implement first"]
async fn shipment_happy_path_converges() {
    // Create a shipment through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a shipment through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn shipment_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}
