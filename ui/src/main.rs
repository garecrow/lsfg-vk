use std::sync::{Arc, OnceLock, RwLock};

use adw;
use gtk::{gio, prelude::*};

mod config;
mod ui;
mod utils;
mod wrapper;

const APP_ID: &str = "gay.pancake.lsfg-vk-ui";

#[derive(Debug)]
struct State {
    selected_game: Option<usize>,
}

static STATE: OnceLock<Arc<RwLock<State>>> = OnceLock::new();

fn main() {
    gio::resources_register_include!("lsfg-vk.gresource").expect("Failed to register resources");
    config::load_config().expect("Failed to load configuration");

    adw::init().expect("Failed to initialize libadwaita");

    // prepare the application state
    STATE
        .set(Arc::new(RwLock::new(State {
            selected_game: None,
        })))
        .expect("Failed to set application state");

    // start the application
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build);
    app.run();
}
