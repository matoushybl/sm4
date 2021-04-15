use byteorder::{ByteOrder, LittleEndian};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;
use device_query::{DeviceQuery, DeviceState, Keycode};
use socketcan::canopen::CANOpenNodeCommand::SendPDO;
use socketcan::canopen::{CANOpen, CANOpenNodeMessage, PDO};
use socketcan::{CANFrame, CANSocket};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DRIVER_ID: u8 = 0x01;

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    let bus = CANOpen::new("can0", Some(50000)).expect("Failed to access the selected CAN bus.");
    let device = bus.create_device(0x01);
    let receiver = device.get_receiver();
    let sender = device.get_sender();

    let target_speed = Arc::new(Mutex::new(0.0));

    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(
            r#"ctrl + c to exit, j, k to control the first motor, n, m to control the second motor"#
        )
    )
        .unwrap();

    std::thread::spawn({
        let sender = sender.clone();
        let receiver = receiver.clone();
        let running = running.clone();
        let target_speed = target_speed.clone();
        move || {
            let mut voltage = 0u16;
            let mut temperature = 0u16;

            while running.load(Ordering::SeqCst) {
                if let Ok(Some(frame)) = receiver
                    .recv()
                    .map(|frame| Option::<CANOpenNodeMessage>::from(frame))
                {
                    match frame {
                        CANOpenNodeMessage::SyncReceived => {}
                        CANOpenNodeMessage::PDOReceived(pdo, data, len) => match pdo {
                            PDO::PDO1 => {
                                voltage = LittleEndian::read_u16(&data);
                                temperature = LittleEndian::read_u16(&data[2..]);
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

                execute!(stdout, cursor::MoveTo(0, 1), Clear(ClearType::CurrentLine)).unwrap();
                stdout.flush();
                stdout.write(
                    "| target (rad/s) | value (rad/s) | current (mA) | voltage (V) | temp (C) |\n"
                        .as_bytes(),
                );

                execute!(stdout, cursor::MoveTo(0, 2), Clear(ClearType::CurrentLine)).unwrap();
                stdout.write(
                    format!(
                        "|{:^16.3}|{:^15.3}|{:^14.3}|{:^13.3}|{:^10.3}|\n",
                        *target_speed.lock().unwrap(),
                        0.0,
                        0.0,
                        voltage as f32 / 1000.0,
                        temperature as f32 / 10.0
                    )
                        .as_bytes(),
                );
            }
        }
    });

    std::thread::spawn({
        let target_speed = target_speed.clone();
        let running = running.clone();
        move || {
            while running.load(Ordering::SeqCst) {
                match read().unwrap() {
                Event::Key(KeyEvent {
                               code: KeyCode::Char('k'),
                               modifiers: _,
                           }) => {
                    // control.lock().unwrap().first += 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('j'),
                               modifiers: _,
                           }) => {
                    // control.lock().unwrap().first -= 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('n'),
                               modifiers: _,
                           }) => {
                    // control.lock().unwrap().second -= 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('m'),
                               modifiers: _,
                           }) => {
                    // control.lock().unwrap().second += 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('o'),
                               modifiers: _,
                           }) => {
                    // status.lock().unwrap().step -= 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('p'),
                               modifiers: _,
                           }) => {
                    // status.lock().unwrap().step += 1.0;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('u'),
                               modifiers: _,
                           }) => {
                    // status.lock().unwrap().freq -= 1;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('i'),
                               modifiers: _,
                           }) => {
                    // status.lock().unwrap().freq += 1;
                }
                Event::Key(KeyEvent {
                               code: KeyCode::Char('c'),
                               modifiers: KeyModifiers::CONTROL,
                           }) => {
                    running.store(false, Ordering::SeqCst);
                    break;
                }
                _ => (),
            }
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        let mut buffer = [0u8; 8];

        LittleEndian::write_f32(&mut buffer[0..4], *target_speed.lock().unwrap());
        // sender.send(SendPDO(0x09, PDO::PDO1, buffer, 4).into());
        std::thread::sleep(Duration::from_millis(50));
    }
}