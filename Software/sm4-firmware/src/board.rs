pub mod config {
    pub const CAN_ID: u8 = 0x01;
    pub const SENSE_R: f32 = 0.22;
    pub const MICROSTEPS: u32 = 16;
    pub const STEPS_PER_REV: u32 = 200;
    pub const ENCODER_RESOLUTION: u32 = MICROSTEPS * STEPS_PER_REV;
}

pub mod definitions {
    use crate::prelude::*;
    use sm4_shared::prelude::*;
    use stm32f4xx_hal::dac::{C1, C2};
    use stm32f4xx_hal::gpio::gpioa::{PA1, PA11, PA12, PA3, PA4, PA5, PA6, PA7, PA8, PA9};
    use stm32f4xx_hal::gpio::gpiob::{PB1, PB10, PB11, PB12, PB6, PB7, PB8, PB9};
    use stm32f4xx_hal::gpio::gpioc::{PC4, PC5, PC6};
    use stm32f4xx_hal::gpio::{
        Alternate, AlternateOD, Analog, Floating, Input, Output, PullDown, PushPull, AF1, AF10,
        AF3, AF4, AF9,
    };

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

    pub type Step1 = PC6<Alternate<AF3>>; // TODO not sure about AF
    pub type Step2 = PA8<Alternate<AF1>>;

    pub type Err1 = PC5<Input<PullDown>>;
    pub type Err2 = PC4<Input<PullDown>>;

    pub type BatteryVoltage = PB1<Analog>;

    pub type ErrorLED = PB6<Output<PushPull>>;
    pub type StatusLED = PB7<Output<PushPull>>;

    pub type CANRx = PB8<Alternate<AF9>>;
    pub type CANTx = PB9<Alternate<AF9>>;

    pub type SCL = PB10<AlternateOD<AF4>>;
    pub type SDA = PB11<AlternateOD<AF4>>;

    pub type USBDMinus = PA11<Alternate<AF10>>;
    pub type USBDPlus = PA12<Alternate<AF10>>;

    type Axis1Driver = TMC2100<
        StepGeneratorTimer<stm32f4xx_hal::pac::TIM8>,
        Step1,
        Dir1,
        CurrentDACChannel<CurrentRef1Channel>,
    >;
    type Axis2Driver = TMC2100<
        StepGeneratorTimer<stm32f4xx_hal::pac::TIM1>,
        Step2,
        Dir2,
        CurrentDACChannel<CurrentRef2Channel>,
    >;
    type Axis1Encoder =
        StepCounterEncoder<stm32f4xx_hal::pac::TIM5, { super::config::ENCODER_RESOLUTION }>;
    type Axis2Encoder =
        StepCounterEncoder<stm32f4xx_hal::pac::TIM2, { super::config::ENCODER_RESOLUTION }>;

    pub type Axis1 =
        AxisMotionController<Axis1Driver, Axis1Encoder, { super::config::ENCODER_RESOLUTION }>;
    pub type Axis2 =
        AxisMotionController<Axis2Driver, Axis2Encoder, { super::config::ENCODER_RESOLUTION }>;
}
