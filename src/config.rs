use anyhow::Context;
use homedir::get_my_home;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    config_dir: PathBuf,
}

impl Config {
    pub fn default_init() -> anyhow::Result<Self> {
        Ok(Config {
            config_dir: get_default_config_dir()
                .context("while determining the path to the config directory")?,
        })
    }

    pub fn from_custom_dir(dir: PathBuf) -> Config {
        Config { config_dir: dir }
    }

    /// get path to a project template by name considering the user's configured template directories
    pub fn get_template_path(&self, name: &str) -> Option<PathBuf> {
        // for now all templates are in <config_dir>/templates
        let path = self.config_dir.join("templates").join(name);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
}

// get the config directory, usually $XDG_CONFIG_HOME/cuber
fn get_default_config_dir() -> anyhow::Result<PathBuf> {
    Ok(std::env::var("XDG_CONFIG_HOME")
        .map(|xdg_conf_home| xdg_conf_home.into())
        .unwrap_or(
            get_my_home()?
                .context("could not determine user's home directory")?
                .join(".config"),
        )
        .join("cuber"))
}
