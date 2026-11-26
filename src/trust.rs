use std::path::PathBuf;

use gpui_shell::{Capabilities, ExecuteGrant, policy};

use crate::app::App;
use crate::home::Home;

pub fn grant_everything(home: &Home, app: &App) -> Result<(), String> {
    let data = home.data(app.name());
    std::fs::create_dir_all(&data)
        .map_err(|error| format!("cannot create {}: {error}", data.display()))?;
    let everywhere = || [PathBuf::from("/")];
    let capabilities = Capabilities::new()
        .read_roots(everywhere())
        .write_roots(everywhere())
        .execute(ExecuteGrant::Unrestricted)
        .storage(true)
        .clipboard_read(true)
        .clipboard_write(true)
        .exit(true);
    let id = app.id();
    policy::update_default(|policy| {
        policy
            .with_application(&id)
            .with_storage_path(data.join("store.json"))
            .with_data_dir(data)
            .with_capabilities(capabilities)
    });
    Ok(())
}
