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

static PWM:u32 = 2;
static LED:u32 = 13;

static PWM_LOW:u32 = 0;
static PWM_HIGH:u32 = 16;

#[no_mangle]
pub fn main() {
    init();
    delay(1);
    pinMode(LED, OUTPUT);
    digitalWrite(LED, LOW);
    analogWrite(PWM, PWM_LOW);

    loop {
        analogWrite(PWM, PWM_HIGH);
        digitalWrite(LED, HIGH);
        delay(100);
        analogWrite(PWM, PWM_LOW);
        digitalWrite(LED, LOW);
        delay(100);

        analogWrite(PWM, PWM_HIGH);
        digitalWrite(LED, HIGH);
        delay(1000);
        analogWrite(PWM, PWM_LOW);
        digitalWrite(LED, LOW);
        delay(1000);
  }
}
