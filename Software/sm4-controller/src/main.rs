mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use crate::ui::UI;
use std::env::args;

fn build_ui(application: &gtk::Application) {
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

    application.connect_activate(|app| {
        build_ui(app);
    });

    let (sender, receiver) = crossbeam::channel::unbounded::<()>();
    std::thread::spawn({
        move || loop {
            while let Ok(a) = receiver.recv() {}
        }
    });

    application.run(&args().collect::<Vec<_>>());
}
