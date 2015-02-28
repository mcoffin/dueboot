#![crate_type = "lib"]

#![feature(asm)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(core)]

#![no_std]

extern crate core;

use core::option::Option;
use core::option::Option::{Some, None};
use core::slice::SliceExt;

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

static LED_PIN:u32 = 13;

#[no_mangle]
pub fn main() {
    init();
    delay(1);

    let led = Pin::new(LED_PIN, PinMode::Output);
    led.digital_write(LOW);

    let delays: &[u32] = &[100, 1000];

    loop {
        for &d in delays.iter() {
            led.digital_write(HIGH);
            delay(d);
            led.digital_write(LOW);
            delay(d);
        }
    }
}
