use assert_fs::prelude::*;
use predicates::prelude::*;

use crate::actions::init_action::*;
use crate::app_context::*;
use crate::config::Config;
use std::path::PathBuf;

use super::Action;

#[test]
fn test_init_lib() {
    let tempdir = assert_fs::TempDir::new().unwrap();
    std::env::set_current_dir(&tempdir).unwrap();
    let ctx = AppContext::custom(Config::from_custom_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("testing")
            .join("test_config"),
    ));
    let outcome = InitAction {
        project_name: "test_lib".to_string(),
    }
    .perform(&ctx);
    assert!(
        outcome.is_ok(),
        "Error: {}",
        outcome.err().unwrap().to_string()
    );
    tempdir.child("test_lib").assert(predicate::path::exists());
}
