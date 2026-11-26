use std::path::PathBuf;

pub struct Home {
    root: PathBuf,
    applications: PathBuf,
}

impl Home {
    pub fn from_environment() -> Self {
        let root = match std::env::var_os("BMO_HOME").filter(|value| !value.is_empty()) {
            Some(root) => PathBuf::from(root),
            None => user_home().join(".config").join("bmo"),
        };
        let applications = match std::env::var_os("BMO_APPLICATIONS").filter(|value| !value.is_empty()) {
            Some(applications) => PathBuf::from(applications),
            None => user_home().join("Applications"),
        };
        Self { root, applications }
    }

    pub fn applications(&self) -> &PathBuf {
        &self.applications
    }

    pub fn app(&self, name: &str) -> PathBuf {
        self.root.join("apps").join(name)
    }

    pub fn share_with_runtime(&self) {
        unsafe { std::env::set_var("GPUI_SHELL_HOME", &self.root) };
    }
}

fn user_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}
