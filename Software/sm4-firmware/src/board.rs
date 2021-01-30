use stm32f4xx_hal::dac::{C1, C2};
use stm32f4xx_hal::gpio::gpioa::{PA1, PA11, PA12, PA3, PA4, PA5, PA6, PA7, PA8, PA9};
use stm32f4xx_hal::gpio::gpiob::{PB1, PB10, PB11, PB12, PB6, PB7, PB8, PB9};
use stm32f4xx_hal::gpio::gpioc::{PC4, PC5, PC6};
use stm32f4xx_hal::gpio::{
    Alternate, AlternateOD, Analog, Floating, Input, Output, PullDown, PushPull, AF1, AF10, AF4,
    AF8,
};

pub mod prelude {
    pub use super::*;
}

pub type Dir1 = PA1<Output<PushPull>>;
pub type Dir2 = PB12<Output<PushPull>>;

pub type Mode1 = PA3<Input<Floating>>;
pub type Mode2 = PA9<Input<Floating>>;

pub type CurrentRef1Pin = PA4<Analog>;
pub type CurrentRef1Channel = C1;
pub type CurrentRef2Pin = PA5<Analog>;
pub type CurrentRef2Channel = C2;

pub type En1 = PA6<Input<Floating>>;
pub type En2 = PA7<Input<Floating>>;

pub type Step1 = PC6<Alternate<AF1>>; // TODO not sure about AF
pub type Step2 = PA8<Alternate<AF1>>;

pub type Err1 = PC5<Input<PullDown>>;
pub type Err2 = PC4<Input<PullDown>>;

pub type BatteryVoltage = PB1<Analog>;

pub type ErrorLED = PB6<Output<PushPull>>;
pub type StatusLED = PB7<Output<PushPull>>;

pub type CANRx = PB8<Alternate<AF8>>;
pub type CANTx = PB9<Alternate<AF8>>;

pub type SCL = PB10<AlternateOD<AF4>>;
pub type SDA = PB11<AlternateOD<AF4>>;

pub type USBDMinus = PA11<Alternate<AF10>>;
pub type USBDPlus = PA12<Alternate<AF10>>;

pub const SENSE_R: f32 = 0.22;
pub const MICROSTEPS: f32 = 16.0;

pub struct GPIO {
    pub dir1: Dir1,
    pub dir2: Dir2,
    pub mode1: Mode1,
    pub mode2: Mode2,
    pub ref1: CurrentRef1Pin,
    pub ref2: CurrentRef2Pin,
    pub en1: En1,
    pub en2: En2,
    pub step1: Step1,
    pub step2: Step2,
    pub err1: Err1,
    pub err2: Err2,
    pub battery_voltage: BatteryVoltage,
    pub error_led: ErrorLED,
    pub status_led: StatusLED,
    pub can_rx: CANRx,
    pub can_tx: CANTx,
    pub scl: SCL,
    pub sda: SDA,
    pub usb_minus: USBDMinus,
    pub usb_plus: USBDPlus,
}

impl GPIO {
    pub fn configure(
        gpioa: stm32f4xx_hal::gpio::gpioa::Parts,
        gpiob: stm32f4xx_hal::gpio::gpiob::Parts,
        gpioc: stm32f4xx_hal::gpio::gpioc::Parts,
    ) -> Self {
        Self {
            dir1: gpioa.pa1.into_push_pull_output(),
            dir2: gpiob.pb12.into_push_pull_output(),
            mode1: gpioa.pa3.into_floating_input(),
            mode2: gpioa.pa9.into_floating_input(),
            ref1: gpioa.pa4.into_analog(),
            ref2: gpioa.pa5.into_analog(),
            en1: gpioa.pa6.into_floating_input(),
            en2: gpioa.pa7.into_floating_input(),
            step1: gpioc.pc6.into_alternate_af1(),
            step2: gpioa.pa8.into_alternate_af1(),
            err1: gpioc.pc5.into_pull_down_input(),
            err2: gpioc.pc4.into_pull_down_input(),
            battery_voltage: gpiob.pb1.into_analog(),
            error_led: gpiob.pb6.into_push_pull_output(),
            status_led: gpiob.pb7.into_push_pull_output(),
            can_rx: gpiob.pb8.into_alternate_af8(),
            can_tx: gpiob.pb9.into_alternate_af8(),
            scl: gpiob.pb10.into_alternate_af4_open_drain(),
            sda: gpiob.pb11.into_alternate_af4_open_drain(),
            usb_minus: gpioa.pa11.into_alternate_af10(),
            usb_plus: gpioa.pa12.into_alternate_af10(),
        }
    }
}
