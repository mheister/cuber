use std::path::PathBuf;

use crate::app_context::AppContext;
use anyhow::{anyhow, Context};

pub struct InitAction {
    pub project_name: String,
}

impl super::Action for InitAction {
    fn perform(self, ctx: &AppContext) -> anyhow::Result<()> {
        match ctx.config.get_template_path("lib") {
            Some(template_path) => {
                println!(
                    "Initializing project {} from template {}",
                    self.project_name,
                    template_path.display()
                );
                // copy path to new project dir
                let project_path =
                    std::env::current_dir().unwrap().join(&self.project_name);
                if project_path.exists() {
                    return Err(anyhow!("Folder {} already exists", self.project_name));
                }
                copy_directory_recursively(&template_path, &project_path).context(
                    format!(
                        "Copying from template '{}' to project path '{}'",
                        &template_path.display(),
                        &project_path.display()
                    ),
                )?;
                Ok(())
            }
            None => {
                // TODO: Hint user towards config directory
                return Err(anyhow!("Template 'lib' for library projects not found!"));
            }
        }
    }
}

fn copy_directory_recursively(from: &PathBuf, to: &PathBuf) -> anyhow::Result<()> {
    let mut q = std::fs::read_dir(&from)?.collect::<Vec<_>>();
    while !q.is_empty() {
        let entry = q.pop().unwrap()?;
        if let Ok(rel) = entry.path().strip_prefix(&from) {
            let target_path = &to.join(&rel);
            if let Some(target_dir) = target_path.parent() {
                if !target_dir.is_dir() {
                    std::fs::create_dir_all(target_dir)?;
                }
            }
            dbg!(&entry, &target_path);
            std::fs::copy(entry.path(), target_path)?;
        }
    }
    Ok(())
}
