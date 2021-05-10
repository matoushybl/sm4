use crate::board::definitions::*;

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
            step1: gpioc.pc6.into_alternate_af3(),
            step2: gpioa.pa8.into_alternate_af1(),
            err1: gpioc.pc5.into_pull_down_input(),
            err2: gpioc.pc4.into_pull_down_input(),
            battery_voltage: gpiob.pb1.into_analog(),
            error_led: gpiob.pb6.into_push_pull_output(),
            status_led: gpiob.pb7.into_push_pull_output(),
            can_rx: gpiob.pb8.into_alternate_af9(),
            can_tx: gpiob.pb9.into_alternate_af9(),
            scl: gpiob.pb10.into_alternate_af4_open_drain(),
            sda: gpiob.pb11.into_alternate_af4_open_drain(),
            usb_minus: gpioa.pa11.into_alternate_af10(),
            usb_plus: gpioa.pa12.into_alternate_af10(),
        }
    }
}
