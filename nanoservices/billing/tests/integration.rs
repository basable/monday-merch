//! Integration tests on the testkit: a database per test with every
//! migration applied. Skipped without TEST_DATABASE_URL. The `#[ignore]`d
//! ones are templates to un-ignore as the pieces land.

use basable_testkit::TestDb;

#[tokio::test]
async fn migrations_apply_and_schema_exists() {
    let Some(db) = TestDb::from_env().await else { return };
    let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_namespace WHERE nspname = 'nano_billing')")
        .fetch_one(db.migrator())
        .await
        .unwrap();
    assert!(exists, "schema nano_billing must exist after the migrations");
}

#[tokio::test]
#[ignore = "implement first"]
async fn invoice_happy_path_converges() {
    // Create a invoice through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a invoice through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn invoice_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}

#[tokio::test]
#[ignore = "implement first"]
async fn payment_happy_path_converges() {
    // Create a payment through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a payment through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn payment_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}

#[tokio::test]
#[ignore = "implement first"]
async fn vat_return_happy_path_converges() {
    // Create a vat_return through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a vat_return through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn vat_return_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}

#[tokio::test]
#[ignore = "implement first"]
async fn supplier_bill_happy_path_converges() {
    // Create a supplier_bill through the handler, run a
    // `basable_processingobject_testkit::Replica` with `drive_once`, assert
    // the status converged.
    todo!("create a supplier_bill through the handler, run a Replica, assert the status converged");
}

#[tokio::test]
#[ignore = "implement first"]
async fn supplier_bill_teardown_settles_only_on_confirmed_absence() {
    todo!("mark_deleted, drive, assert Delete only after the simulator reports absence");
}
