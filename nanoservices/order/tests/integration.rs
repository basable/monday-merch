//! Integration tests on the testkit: a database per test with every
//! migration applied. Skipped without TEST_DATABASE_URL. The `#[ignore]`d
//! ones are templates to un-ignore as the pieces land.

use basable_testkit::TestDb;

#[tokio::test]
async fn migrations_apply_and_schema_exists() {
    let Some(db) = TestDb::from_env().await else { return };
    let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_namespace WHERE nspname = 'nano_order')")
        .fetch_one(db.migrator())
        .await
        .unwrap();
    assert!(exists, "schema nano_order must exist after the migrations");
}

#[tokio::test]
#[ignore = "implement first"]
async fn quote_happy_path_converges() {
    // Create a quote through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a quote through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn quote_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}

#[tokio::test]
#[ignore = "implement first"]
async fn order_happy_path_converges() {
    // Create a order through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a order through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn order_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}

#[tokio::test]
#[ignore = "implement first"]
async fn production_run_happy_path_converges() {
    // Create a production_run through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a production_run through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn production_run_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}
