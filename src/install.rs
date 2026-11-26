use crate::app::App;
use crate::bundle::Bundle;
use crate::home::Home;

pub fn install(home: &Home, app: &App) -> Result<String, String> {
    app.require()?;
    let bundle = Bundle {
        title: app.title(),
        id: app.id(),
        arguments: vec![
            "run".into(),
            "--dir".into(),
            app.directory().display().to_string(),
            "--packaged".into(),
        ],
    };
    let path = bundle.write_into(home.applications())?;
    Ok(format!("installed {}", path.display()))
}
