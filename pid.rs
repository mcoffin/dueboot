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

pub struct PIDController<T, S> {
    kp: S,
    ki: S,
    kd: S,
    pub setpoint: T,
    pub min_out: T,
    pub max_out: T,
    sample_time: u32,
    last_sample_time: u32,
    integral_term: T,
    last_input: T,
}

impl<T, S> PIDController<T, S> where T: Add<Output=T> + Sub<Output=T> + 
    Mul<S, Output=T> + Div<S, Output=T> +
    Copy + PartialOrd + Default,
    S: Copy + PartialOrd + Default +
    Mul<f32, Output=S> + Div<f32, Output=S> {

    pub fn new(p: S, i: S, d: S,
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

    pub fn set_tuning_params(&mut self, p: S, i: S, d: S) {
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
            self.integral_term = self.integral_term + (error * self.ki);

            self.integral_term = limited!(self.integral_term,
                                          self.min_out, self.max_out);

            let output = (error * self.kp) +
                self.integral_term -
                (d_input * self.kd);

            self.last_input = input;
            self.last_sample_time = millis();

            Some(limited!(output, self.min_out, self.max_out))
        }
    }
}
