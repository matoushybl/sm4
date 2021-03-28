use gio::prelude::*;
use gtk::prelude::*;

use gtk::Orientation;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("SM4-controller");
    window.set_border_width(0);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(720, 520);

    // window.set_decorated(true);

    let sync_label = gtk::Label::new(Some("Sync"));
    let sync_switch = gtk::Switch::new();

    let header = gtk::HeaderBar::new();
    header.set_title(Some("SM4"));
    header.set_show_close_button(true);

    header.pack_start(&sync_label);
    header.pack_start(&sync_switch);

    window.set_titlebar(Some(&header));

    let layout = gtk::Notebook::new();
    let run_page = gtk::Box::new(Orientation::Horizontal, 0);
    run_page.set_homogeneous(true);
    run_page.add(&render_axis_pane());
    run_page.add(&render_axis_pane());

    let config_page = gtk::Box::new(Orientation::Horizontal, 0);
    config_page.set_homogeneous(true);
    config_page.add(&render_axis_config_pane());
    config_page.add(&render_axis_config_pane());

    layout.append_page(&run_page, Some(&gtk::Label::new(Some("Controls"))));
    layout.append_page(&config_page, Some(&gtk::Label::new(Some("Config"))));

    window.add(&layout);

    window.show_all();
}

fn labeled<T: IsA<gtk::Widget>>(text: &str, widget: &T) -> gtk::Box {
    let layout = gtk::Box::new(Orientation::Horizontal, 2);
    let label = gtk::Label::new(Some(text));
    label.set_halign(gtk::Align::Start);

    layout.pack_start(&label, true, true, 10);
    layout.pack_end(widget, false, true, 10);
    layout.set_margin_top(5);
    layout
}

fn render_axis_config_pane() -> gtk::Box {
    let layout = gtk::Box::new(Orientation::Vertical, 5);
    let title = gtk::Label::new(None);
    title.set_markup("<big>Axis 1</big>");
    title.set_margin_top(10);
    layout.add(&title);

    const CURRENT_LOWER: f64 = 0.2;
    const CURRENT_HIGHER: f64 = 2.0;
    const CURRENT_STEP: f64 = 0.05;

    fn new_adjustment(lower: f64, higher: f64, step: f64) -> gtk::Adjustment {
        gtk::Adjustment::new(0.0, lower, higher, step, 0.0, 0.0)
    };

    layout.add(&labeled(
        "Standstill current [A]",
        &gtk::SpinButton::new(
            Some(&new_adjustment(CURRENT_LOWER, CURRENT_HIGHER, CURRENT_STEP)),
            0.05,
            5,
        ),
    ));
    layout.add(&labeled(
        "Accelerating current [A]",
        &gtk::SpinButton::new(
            Some(&new_adjustment(CURRENT_LOWER, CURRENT_HIGHER, CURRENT_STEP)),
            0.05,
            5,
        ),
    ));
    layout.add(&labeled(
        "Constant vel. current [A]",
        &gtk::SpinButton::new(
            Some(&new_adjustment(CURRENT_LOWER, CURRENT_HIGHER, CURRENT_STEP)),
            0.05,
            5,
        ),
    ));

    fn new_pid_adjustment() -> gtk::Adjustment {
        gtk::Adjustment::new(0.0, -5.0, 5.0, 0.05, 0.0, 0.0)
    };
    layout.add(&labeled(
        "Velocity P",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));
    layout.add(&labeled(
        "Velocity I",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));
    layout.add(&labeled(
        "Velocity D",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));

    layout.add(&labeled(
        "Position P",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));
    layout.add(&labeled(
        "Position I",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));
    layout.add(&labeled(
        "Position D",
        &gtk::SpinButton::new(Some(&new_pid_adjustment()), 0.05, 5),
    ));

    layout.add(&labeled(
        "Target acceleration [rps/s]",
        &gtk::SpinButton::new(Some(&new_adjustment(-100.0, 100.0, 0.5)), 0.05, 5),
    ));

    layout.add(&labeled(
        "Velocity feedback control enabled",
        &gtk::Switch::new(),
    ));

    layout
}

fn render_axis_pane() -> gtk::Box {
    let layout = gtk::Box::new(Orientation::Vertical, 5);
    let title = gtk::Label::new(None);
    title.set_markup("<big>Axis 1</big>");
    title.set_margin_top(10);

    layout.add(&title);
    layout.add(&labeled("Enabled", &gtk::Switch::new()));

    let mode = gtk::ListStore::new(&[glib::Type::String]);
    mode.set_value(&mode.append(), 0, &Some("Velocity [rps]").to_value());
    mode.set_value(&mode.append(), 0, &Some("Position [revs]").to_value());

    let combo = gtk::ComboBox::with_model_and_entry(&mode);
    combo.set_entry_text_column(0);
    combo.set_active(Some(0));

    let combo = labeled("Mode", &combo);
    layout.add(&combo);

    layout.add(&labeled(
        "Target velocity",
        &gtk::SpinButton::new(
            Some(&gtk::Adjustment::new(0.0, -5.0, 5.0, 0.05, 0.0, 0.0)),
            0.05,
            3,
        ),
    ));
    layout.add(&labeled(
        "Target position",
        &gtk::SpinButton::new(
            Some(&gtk::Adjustment::new(0.0, -500.0, 500.0, 0.05, 0.0, 0.0)),
            0.05,
            5,
        ),
    ));
    layout.add(&labeled("Actual velocity", &gtk::Label::new(Some("0.0"))));
    layout.add(&labeled(
        "Actual revolutions",
        &gtk::Label::new(Some("0.0")),
    ));
    layout.add(&labeled("Actual angle", &gtk::Label::new(Some("0.0"))));

    layout
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
