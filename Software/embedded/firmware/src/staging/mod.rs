//!
//! Asynchronous serial communication using UART/USART peripherals
//!
//! # Word length
//!
//! By default, the UART/USART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
//! the embedded-hal read and write traits with `u8` as the word type.
//!
//! You can also configure the hardware to use 9 data bits with the `Config` `wordlength_9()`
//! function. After creating a `Serial` with this option, use the `with_u16_data()` function to
//! convert the `Serial<_, _, u8>` object into a `Serial<_, _, u16>` that can send and receive
//! `u16`s.
//!
//! In this mode, the `Serial<_, _, u16>`, `Rx<_, u16>`, and `Tx<_, u16>` structs instead implement
//! the embedded-hal read and write traits with `u16` as the word type. You can use these
//! implementations for 9-bit words.
//!

use core::fmt;
use core::marker::PhantomData;

use embedded_hal::blocking;
use embedded_hal::prelude::*;
use embedded_hal::serial;
use nb::block;

use stm32f4xx_hal::stm32::{RCC, USART1};

/// Serial error
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

pub mod config {
    use stm32f4xx_hal::time::{Bps, U32Ext};

    pub enum WordLength {
        DataBits8,
        DataBits9,
    }

    pub enum Parity {
        ParityNone,
        ParityEven,
        ParityOdd,
    }

    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1,
        #[doc = "0.5 stop bits"]
        STOP0P5,
        #[doc = "2 stop bits"]
        STOP2,
        #[doc = "1.5 stop bits"]
        STOP1P5,
    }

    pub enum DmaConfig {
        None,
        Tx,
        Rx,
        TxRx,
    }

    pub struct Config {
        pub baudrate: Bps,
        pub wordlength: WordLength,
        pub parity: Parity,
        pub stopbits: StopBits,
        pub dma: DmaConfig,
        pub half_duplex: bool,
    }

    impl Config {
        pub fn baudrate(mut self, baudrate: Bps) -> Self {
            self.baudrate = baudrate;
            self
        }

        pub fn parity_none(mut self) -> Self {
            self.parity = Parity::ParityNone;
            self
        }

        pub fn parity_even(mut self) -> Self {
            self.parity = Parity::ParityEven;
            self
        }

        pub fn parity_odd(mut self) -> Self {
            self.parity = Parity::ParityOdd;
            self
        }

        pub fn wordlength_8(mut self) -> Self {
            self.wordlength = WordLength::DataBits8;
            self
        }

        pub fn wordlength_9(mut self) -> Self {
            self.wordlength = WordLength::DataBits9;
            self
        }

        pub fn stopbits(mut self, stopbits: StopBits) -> Self {
            self.stopbits = stopbits;
            self
        }

        pub fn half_duplex(mut self) -> Self {
            self.half_duplex = true;
            self
        }
    }

    #[derive(Debug)]
    pub struct InvalidConfig;

    impl Default for Config {
        fn default() -> Config {
            let baudrate = 19_200_u32.bps();
            Config {
                baudrate,
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
                dma: DmaConfig::None,
                half_duplex: false,
            }
        }
    }
}

pub trait Pins<USART> {}
pub trait PinTx<USART> {}
pub trait PinRx<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;

impl PinTx<USART1> for NoTx {}
impl PinRx<USART1> for NoRx {}

impl PinTx<USART1> for PA9<Alternate<AF7>> {}

/// Serial abstraction
pub struct Serial<USART, PINS, WORD = u8> {
    usart: USART,
    pins: PINS,
    _word: PhantomData<WORD>,
}

/// Serial receiver
pub struct Rx<USART, WORD = u8> {
    _usart: PhantomData<USART>,
    _word: PhantomData<WORD>,
}

/// Serial transmitter
pub struct Tx<USART, WORD = u8> {
    _usart: PhantomData<USART>,
    _word: PhantomData<WORD>,
}

impl<USART, PINS, WORD> Serial<USART, PINS, WORD>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    /*
        StopBits::STOP0P5 and StopBits::STOP1P5 aren't supported when using UART

        STOP_A::STOP1 and STOP_A::STOP2 will be used, respectively
    */
    pub fn new(
        usart: USART,
        pins: PINS,
        config: config::Config,
        clocks: Clocks,
    ) -> Result<Self, config::InvalidConfig> {
        use self::config::*;

        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());

            // Enable clock.
            // bb::set(&rcc.$apbXenr, $rcc_bit);
            USART::enable_clock(rcc);
        }

        let pclk_freq = USART::pclk_freq(&clocks);
        let baud = config.baudrate.0;

        // The frequency to calculate USARTDIV is this:
        //
        // (Taken from STM32F411xC/E Reference Manual,
        // Section 19.3.4, Equation 1)
        //
        // 16 bit oversample: OVER8 = 0
        // 8 bit oversample:  OVER8 = 1
        //
        // USARTDIV =          (pclk)
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // BUT, the USARTDIV has 4 "fractional" bits, which effectively
        // means that we need to "correct" the equation as follows:
        //
        // USARTDIV =      (pclk) * 16
        //            ------------------------
        //            8 x (2 - OVER8) x (baud)
        //
        // When OVER8 is enabled, we can only use the lowest three
        // fractional bits, so we'll need to shift those last four bits
        // right one bit

        // Calculate correct baudrate divisor on the fly
        let (over8, div) = if (pclk_freq / 16) >= baud {
            // We have the ability to oversample to 16 bits, take
            // advantage of it.
            //
            // We also add `baud / 2` to the `pclk_freq` to ensure
            // rounding of values to the closest scale, rather than the
            // floored behavior of normal integer division.
            let div = (pclk_freq + (baud / 2)) / baud;
            (false, div)
        } else if (pclk_freq / 8) >= baud {
            // We are close enough to pclk where we can only
            // oversample 8.
            //
            // See note above regarding `baud` and rounding.
            let div = ((pclk_freq * 2) + (baud / 2)) / baud;

            // Ensure the the fractional bits (only 3) are
            // right-aligned.
            let frac = div & 0xF;
            let div = (div & !0xF) | (frac >> 1);
            (true, div)
        } else {
            return Err(config::InvalidConfig);
        };

        unsafe { (*USART::ptr()).brr.write(|w| w.bits(div)) };

        // Reset other registers to disable advanced USART features
        unsafe { (*USART::ptr()).cr2.reset() };
        unsafe { (*USART::ptr()).cr3.reset() };

        if config.half_duplex {
            unsafe { (*USART::ptr()).cr3.write(|w| w.hdsel().half_duplex()) };
        }

        // Enable transmission and receiving
        // and configure frame
        unsafe {
            (*USART::ptr()).cr1.write(|w| {
                w.ue()
                    .set_bit()
                    .over8()
                    .bit(over8)
                    .te()
                    .set_bit()
                    .re()
                    .set_bit()
                    .m()
                    .bit(match config.wordlength {
                        WordLength::DataBits8 => false,
                        WordLength::DataBits9 => true,
                    })
                    .pce()
                    .bit(match config.parity {
                        Parity::ParityNone => false,
                        _ => true,
                    })
                    .ps()
                    .bit(match config.parity {
                        Parity::ParityOdd => true,
                        _ => false,
                    })
            })
        };

        match config.dma {
            DmaConfig::Tx => unsafe { (*USART::ptr()).cr3.write(|w| w.dmat().enabled()) },
            DmaConfig::Rx => unsafe { (*USART::ptr()).cr3.write(|w| w.dmar().enabled()) },
            DmaConfig::TxRx => unsafe {
                (*USART::ptr())
                    .cr3
                    .write(|w| w.dmar().enabled().dmat().enabled())
            },
            DmaConfig::None => {}
        }

        Ok(Serial {
            usart,
            pins,
            _word: PhantomData,
        }
        .config_stop(config))
    }

    /// Starts listening for an interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().set_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().set_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().set_bit()) },
        }
    }

    /// Stop listening for an interrupt event
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.rxneie().clear_bit()) },
            Event::Txe => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.txeie().clear_bit()) },
            Event::Idle => unsafe { (*USART::ptr()).cr1.modify(|_, w| w.idleie().clear_bit()) },
        }
    }

    /// Return true if the line idle status is set
    pub fn is_idle(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().idle().bit_is_set() }
    }

    /// Return true if the tx register is empty (and can accept data)
    pub fn is_txe(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().txe().bit_is_set() }
    }

    /// Return true if the rx register is not empty (and can be read)
    pub fn is_rxne(&self) -> bool {
        unsafe { (*USART::ptr()).sr.read().rxne().bit_is_set() }
    }

    pub fn split(self) -> (Tx<USART, WORD>, Rx<USART, WORD>) {
        (
            Tx {
                _usart: PhantomData,
                _word: PhantomData,
            },
            Rx {
                _usart: PhantomData,
                _word: PhantomData,
            },
        )
    }
    pub fn release(self) -> (USART, PINS) {
        (self.usart, self.pins)
    }
}

impl<USART, PINS> Serial<USART, PINS, u8>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<USART, PINS, u16> {
        Serial {
            usart: self.usart,
            pins: self.pins,
            _word: PhantomData,
        }
    }
}

impl<USART, PINS> Serial<USART, PINS, u16>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<USART, PINS, u8> {
        Serial {
            usart: self.usart,
            pins: self.pins,
            _word: PhantomData,
        }
    }
}

impl<USART, PINS> serial::Read<u16> for Serial<USART, PINS, u16>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u16, Error> {
        let mut rx: Rx<USART, u16> = Rx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        rx.read()
    }
}

impl<USART, PINS> serial::Read<u8> for Serial<USART, PINS, u8>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let mut rx: Rx<USART, u8> = Rx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        rx.read()
    }
}

impl<USART> serial::Read<u8> for Rx<USART, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        // Delegate to the Read<u16> implementation, then truncate to 8 bits
        let mut rx_u16: Rx<USART, u16> = Rx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        rx_u16.read().map(|word16| word16 as u8)
    }
}

/// Reads 9-bit words from the UART/USART
///
/// If the UART/USART was configured with `WordLength::DataBits9`, the returned value will contain
/// 9 received data bits and all other bits set to zero. Otherwise, the returned value will contain
/// 8 received data bits and all other bits set to zero.
impl<USART> serial::Read<u16> for Rx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u16, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        // Any error requires the dr to be read to clear
        if sr.pe().bit_is_set()
            || sr.fe().bit_is_set()
            || sr.nf().bit_is_set()
            || sr.ore().bit_is_set()
        {
            unsafe { (*USART::ptr()).dr.read() };
        }

        Err(if sr.pe().bit_is_set() {
            nb::Error::Other(Error::Parity)
        } else if sr.fe().bit_is_set() {
            nb::Error::Other(Error::Framing)
        } else if sr.nf().bit_is_set() {
            nb::Error::Other(Error::Noise)
        } else if sr.ore().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.rxne().bit_is_set() {
            // NOTE(unsafe) atomic read from stateless register
            return Ok(unsafe { &*USART::ptr() }.dr.read().dr().bits());
        } else {
            nb::Error::WouldBlock
        })
    }
}

unsafe impl<USART> PeriAddress for Rx<USART, u8>
where
    USART: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

impl<USART, PINS> serial::Write<u16> for Serial<USART, PINS, u16>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let mut tx: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.flush()
    }

    fn write(&mut self, byte: u16) -> nb::Result<(), Self::Error> {
        let mut tx: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.write(byte)
    }
}

impl<USART, PINS> serial::Write<u8> for Serial<USART, PINS, u8>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let mut tx: Tx<USART, u8> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.flush()
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        let mut tx: Tx<USART, u8> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.write(byte)
    }
}

unsafe impl<USART> PeriAddress for Tx<USART, u8>
where
    USART: Instance,
{
    #[inline(always)]
    fn address(&self) -> u32 {
        &(unsafe { &(*USART::ptr()) }.dr) as *const _ as u32
    }

    type MemSize = u8;
}

impl<USART> serial::Write<u8> for Tx<USART, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // Delegate to u16 version
        let mut tx_u16: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx_u16.write(u16::from(word))
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Delegate to u16 version
        let mut tx_u16: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx_u16.flush()
    }
}

/// Writes 9-bit words to the UART/USART
///
/// If the UART/USART was configured with `WordLength::DataBits9`, the 9 least significant bits will
/// be transmitted and the other 7 bits will be ignored. Otherwise, the 8 least significant bits
/// will be transmitted and the other 8 bits will be ignored.
impl<USART> serial::Write<u16> for Tx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).sr.read() };

        if sr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            unsafe { &*USART::ptr() }.dr.write(|w| w.dr().bits(word));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<USART> blocking::serial::Write<u16> for Tx<USART, u16>
where
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, buffer: &[u16]) -> Result<(), Self::Error> {
        for &b in buffer {
            loop {
                match self.write(b) {
                    Err(nb::Error::WouldBlock) => continue,
                    Err(nb::Error::Other(err)) => return Err(err),
                    Ok(()) => break,
                }
            }
        }
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        loop {
            match <Self as serial::Write<u16>>::flush(self) {
                Ok(()) => return Ok(()),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
}

impl<USART> blocking::serial::Write<u8> for Tx<USART, u8>
where
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        for &b in bytes {
            loop {
                match self.write(b) {
                    Err(nb::Error::WouldBlock) => continue,
                    Err(nb::Error::Other(err)) => return Err(err),
                    Ok(()) => break,
                }
            }
        }
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        loop {
            match <Self as serial::Write<u8>>::flush(self) {
                Ok(()) => return Ok(()),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
}

impl<USART, PINS> blocking::serial::Write<u16> for Serial<USART, PINS, u16>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, bytes: &[u16]) -> Result<(), Self::Error> {
        let mut tx: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.bwrite_all(bytes)
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        let mut tx: Tx<USART, u16> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.bflush()
    }
}

impl<USART, PINS> blocking::serial::Write<u8> for Serial<USART, PINS, u8>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    type Error = Error;

    fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut tx: Tx<USART, u8> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.bwrite_all(bytes)
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        let mut tx: Tx<USART, u8> = Tx {
            _usart: PhantomData,
            _word: PhantomData,
        };
        tx.bflush()
    }
}

impl<USART, PINS, WORD> Serial<USART, PINS, WORD>
where
    PINS: Pins<USART>,
    USART: Instance,
{
    fn config_stop(self, config: config::Config) -> Self {
        self.usart.set_stopbits(config.stopbits);
        self
    }
}

use stm32f4xx_hal::pac::uart4 as uart_base;

use stm32f4xx_hal::dma::traits::PeriAddress;
use stm32f4xx_hal::gpio::gpioa::PA9;
use stm32f4xx_hal::gpio::{Alternate, AF7};
use stm32f4xx_hal::rcc::Clocks;

mod private {
    pub trait Sealed {}
}

// Implemented by all USART instances
pub trait Instance: private::Sealed {
    #[doc(hidden)]
    fn ptr() -> *const uart_base::RegisterBlock;
    #[doc(hidden)]
    fn pclk_freq(clocks: &Clocks) -> u32;
    #[doc(hidden)]
    unsafe fn enable_clock(rcc: &stm32f4xx_hal::stm32::rcc::RegisterBlock);
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

macro_rules! halUsart {
    ($(
        $USARTX:ident: ($usartX:ident, $apbXenr:ident, $rcc_bit:expr, $usartXen:ident, $pclkX:ident),
    )+) => {
        $(
            impl private::Sealed for $USARTX {}
            impl Instance for $USARTX {
                fn ptr() -> *const uart_base::RegisterBlock {
                    $USARTX::ptr() as *const _
                }

                fn pclk_freq(clocks: &Clocks) -> u32 {
                    clocks.$pclkX().0
                }

                unsafe fn enable_clock(rcc: &stm32f4xx_hal::stm32::rcc::RegisterBlock) {
                    use stm32f4xx_hal::bb;

                    bb::set(&rcc.$apbXenr, $rcc_bit);
                }

                fn set_stopbits(&self, bits: config::StopBits) {
                    use stm32f4xx_hal::stm32::usart1::cr2::STOP_A;
                    use config::StopBits;

                    self
                        .cr2
                        .write(|w| w.stop().variant(
                            match bits {
                                StopBits::STOP0P5 => STOP_A::STOP0P5,
                                StopBits::STOP1 => STOP_A::STOP1,
                                StopBits::STOP1P5 => STOP_A::STOP1P5,
                                StopBits::STOP2 => STOP_A::STOP2,
                            }
                        ));
                }
            }

            impl<USART, PINS> Serial<USART, PINS>
            where
                PINS: Pins<USART>,
                USART: Instance,
            {
                pub fn $usartX(
                    usart: USART,
                    pins: PINS,
                    config: config::Config,
                    clocks: Clocks,
                ) -> Result<Self, config::InvalidConfig> {
                    Self::new(usart, pins, config, clocks)
                }
            }
        )+
    }
}

halUsart! {
    USART1: (usart1, apb2enr, 4, usart1en, pclk2),
}

impl<USART, PINS> fmt::Write for Serial<USART, PINS>
where
    Serial<USART, PINS>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| block!(self.write(*c)))
            .map_err(|_| fmt::Error)
    }
}

impl<USART> fmt::Write for Tx<USART>
where
    Tx<USART>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| block!(self.write(*c)))
            .map_err(|_| fmt::Error)
    }
}
