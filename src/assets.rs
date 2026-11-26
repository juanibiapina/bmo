use std::borrow::Cow;
use std::path::{Path, PathBuf};

use gpui_shell::AppAssets;
use gpui_shell::anyhow::Result;
use gpui_shell::gpui::{AssetSource, SharedString};

pub struct Assets {
    inside: AppAssets,
}

impl Assets {
    pub fn of(directory: PathBuf) -> Self {
        Self {
            inside: AppAssets::new(directory),
        }
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let anywhere = Path::new(path);
        if anywhere.is_absolute() && anywhere.is_file() {
            return Ok(Some(Cow::Owned(std::fs::read(anywhere)?)));
        }
        self.inside.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        self.inside.list(path)
    }
}
