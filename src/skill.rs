use std::path::PathBuf;

const SKILL: &str = include_str!("skill.md");

/// Keeps the skill that sends agents to `bmo guide` where agents look for
/// skills, so it always matches the installed bmo. A folder that cannot be
/// written is left alone: the skill is a convenience, never a reason to fail.
pub fn refresh() {
    let file = folder().join("bmo").join("SKILL.md");
    if std::fs::read_to_string(&file).is_ok_and(|found| found == SKILL) {
        return;
    }
    if let Some(folder) = file.parent() {
        let _ = std::fs::create_dir_all(folder);
    }
    let _ = std::fs::write(&file, SKILL);
}

fn folder() -> PathBuf {
    match std::env::var_os("BMO_SKILLS").filter(|value| !value.is_empty()) {
        Some(folder) => PathBuf::from(folder),
        None => std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join(".agents")
            .join("skills"),
    }
}
