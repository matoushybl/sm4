use gtk::prelude::*;
use gtk::{ApplicationWindow, HeaderBar, Notebook, Orientation, Switch};

#[derive(Clone)]
pub struct UI {
    header: HeaderBar,
    sync_switch: Switch,
    tabs: Notebook,
}

impl UI {
    pub fn new(window: &ApplicationWindow) -> Self {
        let sync_label = gtk::Label::new(Some("Sync"));
        let sync_switch = gtk::Switch::new();

        let header = gtk::HeaderBar::new();
        header.set_title(Some("SM4"));
        header.set_show_close_button(true);

        header.pack_start(&sync_label);
        header.pack_start(&sync_switch);

        window.set_titlebar(Some(&header));

        let tabs = gtk::Notebook::new();
        let run_page = gtk::Box::new(Orientation::Horizontal, 0);
        run_page.set_homogeneous(true);
        let axis1_control_pane = ControlPaneUI::new("Axis 1");
        let axis2_control_pane = ControlPaneUI::new("Axis 2");
        run_page.add(&axis1_control_pane.container);
        run_page.add(&axis2_control_pane.container);

        let config_page = gtk::Box::new(Orientation::Horizontal, 0);
        config_page.set_homogeneous(true);
        config_page.add(&render_axis_config_pane());
        config_page.add(&render_axis_config_pane());

        tabs.append_page(&run_page, Some(&gtk::Label::new(Some("Controls"))));
        tabs.append_page(&config_page, Some(&gtk::Label::new(Some("Config"))));

        Self {
            header,
            sync_switch,
            tabs,
        }
    }

    pub fn main_widget(&self) -> &gtk::Notebook {
        &self.tabs
    }
}

#[derive(Clone)]
struct ControlPaneUI {
    container: gtk::Box,
    enabled_switch: gtk::Switch,
    mode_combo_box: gtk::ComboBox,
    target_velocity: gtk::SpinButton,
    target_position: gtk::SpinButton,
    actual_velocity: gtk::Label,
    actual_revolutions: gtk::Label,
    actual_angle: gtk::Label,
}

impl ControlPaneUI {
    pub fn new(title: &str) -> Self {
        let layout = gtk::Box::new(Orientation::Vertical, 5);
        let title_label = gtk::Label::new(None);
        title_label.set_markup(format!("<big>{}</big>", title).as_str());
        title_label.set_margin_top(10);

        layout.add(&title_label);
        let enabled_switch = gtk::Switch::new();
        layout.add(&labeled("Enabled", &enabled_switch));

        let mode = gtk::ListStore::new(&[glib::Type::String]);
        mode.set_value(&mode.append(), 0, &Some("Velocity").to_value());
        mode.set_value(&mode.append(), 0, &Some("Position").to_value());

        let mode_combo_box = gtk::ComboBox::with_model_and_entry(&mode);
        mode_combo_box.set_entry_text_column(0);
        mode_combo_box.set_active(Some(0));

        let combo = labeled("Mode", &mode_combo_box);
        layout.add(&combo);

        let target_velocity = gtk::SpinButton::new(
            Some(&gtk::Adjustment::new(0.0, -5.0, 5.0, 0.05, 0.0, 0.0)),
            0.05,
            3,
        );
        layout.add(&labeled("Target velocity", &target_velocity));
        let target_position = gtk::SpinButton::new(
            Some(&gtk::Adjustment::new(0.0, -500.0, 500.0, 0.05, 0.0, 0.0)),
            0.05,
            5,
        );
        layout.add(&labeled("Target position", &target_position));
        let actual_velocity = gtk::Label::new(None);
        layout.add(&labeled("Actual velocity", &actual_velocity));
        let actual_revolutions = gtk::Label::new(None);
        layout.add(&labeled("Actual revolutions", &actual_revolutions));
        let actual_angle = gtk::Label::new(None);
        layout.add(&labeled("Actual angle", &actual_angle));

        Self {
            container: layout,
            enabled_switch,
            mode_combo_box,
            target_velocity,
            target_position,
            actual_velocity,
            actual_revolutions,
            actual_angle,
        }
    }
}

#[derive(Clone)]
struct ConfigPaneUI {}

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
    }

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
    }
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
