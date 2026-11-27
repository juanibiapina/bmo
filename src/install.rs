use crate::app::App;
use crate::bundle::Bundle;
use crate::home::Home;

pub fn install(home: &Home, app: &App) -> Result<String, String> {
    app.require()?;
    let bundle = Bundle {
        title: app.title(),
        id: app.id(),
        home: home.root().clone(),
        arguments: vec!["run".into(), app.name().into()],
    };
    let path = bundle.write_into(home.applications())?;
    Ok(format!("installed {}", path.display()))
}
