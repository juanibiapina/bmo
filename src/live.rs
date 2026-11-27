use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

/// Makes the live copy match the source. An installed app runs from the live
/// copy and watches it, so this is the moment new code reaches a running
/// window. Files that did not change are left alone, and the entry file is
/// written last, so the window reloads once everything it imports is in place.
pub fn sync(source: &Path, live: &Path) -> io::Result<()> {
    let mut wanted = files(source)?;
    let entry = PathBuf::from(entry(source));
    let last = wanted.take(&entry);
    for file in wanted.iter().chain(last.iter()) {
        let contents = std::fs::read(source.join(file))?;
        let target = live.join(file);
        if std::fs::read(&target).is_ok_and(|found| found == contents) {
            continue;
        }
        if let Some(folder) = target.parent() {
            std::fs::create_dir_all(folder)?;
        }
        std::fs::write(target, contents)?;
    }
    wanted.extend(last);
    for file in files(live)?.difference(&wanted) {
        std::fs::remove_file(live.join(file))?;
    }
    Ok(())
}

fn entry(source: &Path) -> String {
    std::fs::read_to_string(source.join("gpui-shell.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|manifest| manifest["entry"].as_str().map(str::to_string))
        .unwrap_or_else(|| "main.js".into())
}

/// Every file under the folder, relative to it. Git's own folder is not code.
fn files(folder: &Path) -> io::Result<BTreeSet<PathBuf>> {
    let mut found = BTreeSet::new();
    let mut waiting = vec![PathBuf::new()];
    while let Some(relative) = waiting.pop() {
        let Ok(entries) = std::fs::read_dir(folder.join(&relative)) else {
            continue;
        };
        for entry in entries {
            let entry = entry?;
            let path = relative.join(entry.file_name());
            if entry.file_name() == ".git" {
                continue;
            }
            if std::fs::metadata(entry.path())?.is_dir() {
                waiting.push(path);
            } else {
                found.insert(path);
            }
        }
    }
    Ok(found)
}
