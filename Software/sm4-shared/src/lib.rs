//! The shared library for the SM4 - dual channel stepper motor controller.
//!
//! This shared library contains the abstractions for motor control and common data structures
//! for interfacing with the control software.

#![cfg_attr(not(test), no_std)]

pub mod canopen;
pub mod encoder;
pub mod float;

use core::marker::PhantomData;

/// This enum represents the direction where the motor is turning when looking at the shaft.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Clockwise
    }
}

impl Direction {
    /// Returns the opposite direction. `CounterClockwise` when `Clockwise` is selected and vice-versa.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::CounterClockwise,
        }
    }

    /// In motor control the direction of rotation is usually denoted by a positive or negative number.
    /// This method returns `1` for `Clockwise` and `-1` for `CounterClockwise`
    pub fn multiplier(&self) -> i32 {
        match self {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        }
    }
}

/// Marker struct denoting the first motor.
/// The struct is meant to be used in trait implementations to provide extra level of type safety.
pub struct Motor1;

/// Marker struct denoting the second motor.
/// The struct is meant to be used in trait implementations to provide extra level of type safety.
pub struct Motor2;

/// Defines a shared interface for methods of setting motor current,
/// be it inbuilt DAC or external one or something completely different.
/// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait CurrentReference<M> {
    /// Sets the reference current to the supplied value
    /// # Arguments
    /// * current - target current in amps
    fn set_current(&mut self, current: f32);
}

/// Defines a shared interface for working with systems that generates steps that control the stepper motors.
/// The stepper generator can be PWM or manual toggling of an output pin.
/// It is worth noting that the API works directly with step frequency, not microstep frequency, to be independent on microstepping.
/// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait StepGenerator<M> {
    /// Sets the output step frequency.
    /// # Arguments
    /// * freq - frequency of whole steps output by the generator **not microsteps**.
    fn set_step_frequency(&mut self, freq: f32);
}

/// The `DirectionController` interface specifies a way of controlling direction where the motor is turning.
/// /// The `M` parameter is used to distinguish between motors, providing additional type safety.
pub trait DirectionController<M> {
    fn set_direction(&mut self, direction: Direction);
}

// FIXME TODO WIP section
pub trait StepCounter<M> {
    fn reset_steps(&mut self);

    fn get_steps(&mut self) -> i64;

    fn set_direction(&mut self, direction: Direction);
}

const STANDSTILL_CURRENT: f32 = 0.2;
const ACCELERATION_CURRENT: f32 = 0.7;
const CONSTANT_SPEED_CURRENT: f32 = 0.4;

pub struct Driver<M, G, D, C, R> {
    _motor: PhantomData<M>,
    generator: G,
    dir: D,
    counter: C,
    reference: R,
    current_step_frequency: f32,
}

pub trait StepperDriver {
    fn set_step_frequency(&mut self, freq: f32);
    fn get_steps(&mut self) -> i64;
    fn reset_step_counter(&mut self);
}

impl<M, G, D, C, R> Driver<M, G, D, C, R>
where
    G: StepGenerator<M>,
    D: DirectionController<M>,
    C: StepCounter<M>,
    R: CurrentReference<M>,
{
    pub fn new(generator: G, dir: D, counter: C, mut reference: R) -> Self {
        reference.set_current(STANDSTILL_CURRENT);
        Self {
            _motor: Default::default(),
            generator,
            dir,
            counter,
            reference,
            current_step_frequency: 0.0,
        }
    }
}

impl<M, G, D, C, R> StepperDriver for Driver<M, G, D, C, R>
where
    G: StepGenerator<M>,
    D: DirectionController<M>,
    C: StepCounter<M>,
    R: CurrentReference<M>,
{
    fn set_step_frequency(&mut self, freq: f32) {
        let direction = if freq < 0.0 {
            Direction::CounterClockwise
        } else {
            Direction::Clockwise
        };
        self.generator.set_step_frequency(freq);
        self.dir.set_direction(direction);
        self.counter.set_direction(direction);

        if freq == 0.0 {
            self.reference.set_current(STANDSTILL_CURRENT);
        } else if float::fabs(freq - self.current_step_frequency) < core::f32::EPSILON {
            self.reference.set_current(CONSTANT_SPEED_CURRENT);
        } else {
            self.reference.set_current(ACCELERATION_CURRENT);
        }

        self.current_step_frequency = freq;
    }

    fn get_steps(&mut self) -> i64 {
        self.counter.get_steps()
    }

    fn reset_step_counter(&mut self) {
        self.counter.reset_steps();
    }
}
