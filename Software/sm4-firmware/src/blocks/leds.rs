use crate::board::definitions::{ErrorLED, StatusLED};
use blinq::Blinq;

pub struct LEDs {
    status_led: Blinq<blinq::consts::U8, StatusLED>,
    error_led: Blinq<blinq::consts::U8, ErrorLED>,
}

impl LEDs {
    pub fn new(status_led: StatusLED, error_led: ErrorLED) -> Self {
        Self {
            status_led: Blinq::new(status_led, false),
            error_led: Blinq::new(error_led, false),
        }
    }

    pub fn tick(&mut self) {
        self.status_led.step();
        self.error_led.step();
    }

    pub fn signalize_sync(&mut self) {
        self.status_led.enqueue(blinq::patterns::morse::SOS);
    }
}
