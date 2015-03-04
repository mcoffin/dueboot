use core::cmp::PartialOrd;
use core::default::Default;
use core::ops::{Add, Sub, Mul, Div};
use core::marker::Copy;
use core::option::Option;
use core::option::Option::{None, Some};

use super::arduino::millis;

macro_rules! limited {
    ($x:expr, $min:expr, $max:expr) => (
        if $x > $max {
            $max
        } else if $x < $min {
            $min
        } else {
            $x
        }
    );
}

pub struct PIDController<T> {
    kp: T,
    ki: T,
    kd: T,
    pub setpoint: T,
    pub min_out: T,
    pub max_out: T,
    sample_time: u32,
    last_sample_time: u32,
    integral_term: T,
    last_input: T,
}

impl<T> PIDController<T> where T: Add<Output=T> + Sub<Output=T> + 
    Div<f32, Output=T> + Div<Output=T> +
    Mul<f32, Output=T> + Mul<Output=T> +
    Copy + PartialOrd + Default {
    pub fn new(p: T, i: T, d: T,
               min: T, max: T,
               sample_time: u32) -> Self {
        let mut c = PIDController {
            kp: p,
            ki: i,
            kd: d,
            setpoint: Default::default(),
            min_out: min,
            max_out: max,
            sample_time: sample_time,
            last_sample_time: 0,
            integral_term: Default::default(),
            last_input: Default::default(),
        };
        c.set_tuning_params(p, i, d);
        c
    }

    pub fn set_tuning_params(&mut self, p: T, i: T, d: T) {
        self.kp = p;
        self.ki = i * (self.sample_time as f32);
        self.kd = d / (self.sample_time as f32);
    }

    pub fn set_sample_time(&mut self, sample_time: u32) {
        let p = self.kp;
        let i = self.ki / (self.sample_time as f32);
        let d = self.kd * (self.sample_time as f32);

        self.sample_time = sample_time;
        self.set_tuning_params(p, i, d);
    }

    fn since_sample(&self) -> u32 {
        millis() - self.last_sample_time
    }

    pub fn compute(&mut self, input: T) -> Option<T> {
        if self.since_sample() < self.sample_time {
            None
        } else {
            let error = self.setpoint - input;

            let d_input = self.last_input - input;
            self.integral_term = self.integral_term + (self.ki * error);

            self.integral_term = limited!(self.integral_term,
                                          self.min_out, self.max_out);

            let output = (self.kp * error) +
                self.integral_term -
                (self.kd * d_input);

            self.last_input = input;
            self.last_sample_time = millis();

            Some(limited!(output, self.min_out, self.max_out))
        }
    }
}
