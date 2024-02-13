pub mod init_action;
pub use init_action::*;

#[cfg(test)]
mod init_action_tests;

use crate::app_context::AppContext;

pub trait Action {
    fn perform(self, context: &AppContext) -> anyhow::Result<()>;
}