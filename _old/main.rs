use wry::Result;
// use std::env;

// mod register_icon_for_snote;
mod create_app;
mod structs;


fn main() -> Result<()> 
{

    let app = structs::App {
        title: "Silvernote",
        url: "https://app.silvernote.fr",
        icon_path: "icon.png"
    };

    // let exe_path = env::current_exe()?;
    // let exe_dir = exe_path.parent().unwrap();
    // let icon_path = exe_dir.join("icon.ico");

    // register_icon_for_snote::launch(icon_path.to_str().unwrap())?;
    create_app::launch(app)?;

    Ok(())

}