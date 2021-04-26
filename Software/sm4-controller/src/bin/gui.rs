use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

use byteorder::{ByteOrder, LittleEndian};
use sm4_controller::gui::ui::UI;
use sm4_controller::State;
use socketcan::canopen::{CANOpen, CANOpenNodeMessage, PDO};
use std::sync::{Arc, Mutex};

fn build_ui(application: &gtk::Application, state: Arc<Mutex<State>>) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("SM4-controller");
    window.set_border_width(0);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(720, 520);

    let ui = UI::new(&window);

    window.add(ui.main_widget());

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    let state = Arc::new(Mutex::new(State::default()));

    let state_clone = state.clone();
    application.connect_activate(move |app| {
        build_ui(app, state_clone.clone());
    });

    application.run(&args().collect::<Vec<_>>());
}
