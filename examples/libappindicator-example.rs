extern crate gtk;
extern crate libappindicator;
use std::env;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    let mut path = env::current_dir().expect("");
    path.push("./examples/rust-logo-64x64-blk.png");
    indicator.set_icon_full(path.to_str().unwrap(), "icon");
    let mut m = gtk::Menu::new();
    let mi = gtk::CheckMenuItem::new_with_label("Hello RUST");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);
    indicator.set_menu(&mut m);
    m.show_all();
    gtk::main();
}
