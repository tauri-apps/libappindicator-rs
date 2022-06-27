use std::{
  env,
  path::{Path, PathBuf},
};

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
  gtk::init().unwrap();
  let mut indicator = AppIndicator::new("libappindicator test application", "");
  indicator.set_status(AppIndicatorStatus::Active);

  let icon_path = match env::var("TRAY_ICON_DIR") {
    Ok(dir) => PathBuf::from(dir),
    _ => Path::new(env!("CARGO_MANIFEST_DIR")).join("examples"),
  };

  let icon_name = match env::var("TRAY_ICON_NAME") {
    Ok(name) => name,
    _ => "rust-logo".to_string(),
  };

  indicator.set_icon_theme_path(icon_path.to_str().unwrap());
  indicator.set_icon_full(&icon_name, "icon");
  let mut m = gtk::Menu::new();
  let mi = gtk::CheckMenuItem::with_label("Hello Rust!");
  mi.connect_activate(|_| {
    gtk::main_quit();
  });
  m.append(&mi);
  indicator.set_menu(&mut m);
  m.show_all();
  gtk::main();
}
