#![crate_type = "lib"]

#![feature(asm)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(core)]

#![no_std]

extern crate core;

use arduino::{init, delay, pinMode, digitalWrite, analogWrite, LOW, HIGH, OUTPUT};
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

pub struct LED {
    pin: u32,
}

impl LED {
    pub fn new(pin: u32) -> LED {
        let l = LED {
            pin: pin,
        };
        l.set_pin_mode();
        l
    }

    fn set_pin_mode(&self) {
        pinMode(self.pin, OUTPUT);
    }

    pub fn set(&self, val: u8) {
        digitalWrite(self.pin, val);
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
    let led = LED::new(LED_PIN);
    led.set(LOW);
    analogWrite(PWM, PWM_LOW);

    loop {
        analogWrite(PWM, PWM_HIGH);
        led.set(HIGH);
        delay(100);
        analogWrite(PWM, PWM_LOW);
        led.set(LOW);
        delay(100);

        analogWrite(PWM, PWM_HIGH);
        led.set(HIGH);
        delay(1000);
        analogWrite(PWM, PWM_LOW);
        led.set(LOW);
        delay(1000);
    }
}
