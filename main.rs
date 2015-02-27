#![crate_type = "lib"]

#![feature(asm)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(core)]

#![no_std]

extern crate core;

use core::option::Option;
use core::option::Option::{Some, None};

use arduino::{init, delay, pinMode, digitalWrite, digitalRead, analogRead, analogWrite, LOW, HIGH, OUTPUT, INPUT};
mod arduino;

#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[allow(unused_variables)]
/*
fn panic_fmt(args: core::fmt::Arguments,
            file: &str,
            line: usize) ->  ! {
    loop {}
}
*/
fn panic_fmt(args: core::fmt::Arguments,
            file: &'static str,
            line: usize) {
    loop {
    }
}

pub enum PinMode {
    Input,
    Output,
}

pub struct Pin {
    pin: u32,
}

impl Pin {
    pub fn new(pin: u32, mode: PinMode) -> Self {
        let l = Pin {
            pin: pin,
        };
        l.set_pin_mode(mode);
        l
    }

    pub fn set_pin_mode(&self, mode: PinMode) {
        let m = match mode {
            PinMode::Input => INPUT,
            PinMode::Output => OUTPUT,
        };
        pinMode(self.pin, m);
    }

    pub fn digital_write(&self, val: u8) {
        digitalWrite(self.pin, val);
    }

    pub fn digital_read(&self) -> i32 {
        digitalRead(self.pin)
    }

    pub fn analog_write(&self, val: u32) {
        analogWrite(self.pin, val);
    }

    pub fn analog_read(&self) -> i32 {
        analogRead(self.pin)
    }
}

pub struct PIDController {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    setpoint: Option<f64>,
}

impl PIDController {
    pub fn new(p: f64, i: f64, d: f64) -> PIDController {
        PIDController {
            kp: p,
            ki: i,
            kd: d,
            setpoint: None,
        }
    }

    pub fn clear(&mut self) {
        self.setpoint = None;
    }

    pub fn reset(&mut self, setpoint: f64) {
        self.setpoint = Some(setpoint);
    }

    pub fn tick(&mut self, input: f64) -> Option<f64> {
        match self.setpoint {
            None => None,
            Some(x) => Some(x),
        }
    }
}

static PWM:u32 = 2;
static LED_PIN:u32 = 13;

static PWM_LOW:u32 = 0;
static PWM_HIGH:u32 = 16;

#[no_mangle]
pub fn main() {
    init();
    delay(1);

    let mut c = PIDController::new(1.0, 1.0, 1.0);

    let led = Pin::new(LED_PIN, PinMode::Output);
    led.digital_write(LOW);

    loop {
        led.digital_write(HIGH);
        delay(100);
        led.digital_write(LOW);
        delay(100);

        led.digital_write(HIGH);
        delay(1000);
        led.digital_write(LOW);
        delay(1000);

        c.tick(0.0);
    }
}
