use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

use sm4_controller::ui::UI;
use std::sync::{Arc, Mutex};
use sm4_controller::State;
use socketcan::canopen::{CANOpen, CANOpenNodeMessage, PDO};
use byteorder::{LittleEndian, ByteOrder};

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

    let bus = CANOpen::new("can0", Some(50000)).expect("Failed to access the selected CAN bus.");
    let device = bus.create_device(0x01);
    let receiver = device.get_receiver();
    let sender = device.get_sender();

    std::thread::spawn({
        let sender = sender.clone();
        let receiver = receiver.clone();
        let state = state.clone();
        move || {
            loop {
                if let Ok(Some(frame)) = receiver
                    .recv()
                    .map(|frame| Option::<CANOpenNodeMessage>::from(frame))
                {
                    match frame {
                        CANOpenNodeMessage::SyncReceived => {}
                        CANOpenNodeMessage::PDOReceived(pdo, data, len) => match pdo {
                            PDO::PDO1 => {
                                state.lock().unwrap().voltage = LittleEndian::read_u16(&data) as f32 / 1000.0;
                                state.lock().unwrap().temperature = LittleEndian::read_u16(&data[2..]) as f32 / 10.0;
                            }
                            PDO::PDO2 => {
                                // voltage = LittleEndian::read_f32(&data);
                                // temperature = LittleEndian::read_f32(&data[4..]);
                            }
                            PDO::PDO3 => {}
                            PDO::PDO4 => {}
                        },
                        CANOpenNodeMessage::NMTReceived(_) => {}
                        CANOpenNodeMessage::SDOReceived(_, _, _, _, _) => {}
                    }
                }
            }
        }
    });

    application.run(&args().collect::<Vec<_>>());
}
