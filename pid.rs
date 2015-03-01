use core::cmp::PartialOrd;
use core::marker::Copy;
use core::default::Default;
use super::arduino::{millis};

pub struct PIDController<'a, 'b> {
    pub output: &'a mut f32,
    pub input: &'b mut f32,
    kp: f32,
    ki: f32,
    kd: f32,
    pub setpoint: f32,
    pub min_out: f32,
    pub max_out: f32,
    sample_time: u32,
    last_sample_time: u32,
    integral_term: f32,
    last_input: f32,
}

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

fn enforce_limit<T: PartialOrd + Copy>(a: &mut T, min: T, max: T) {
    *a = limited!(*a, min, max);
}

impl<'a, 'b> PIDController<'a, 'b> {
    pub fn new(input: &'b mut f32, output: &'a mut f32,
               p: f32, i: f32, d: f32,
               min: f32, max: f32,
               sample_time: u32) -> Self {
        let mut c = PIDController {
            output: output,
            input: input,
            kp: p,
            ki: i,
            kd: d,
            setpoint: Default::default(),
            min_out: min,
            max_out: max,
            sample_time: sample_time,
            last_sample_time: 0,
            integral_term: 0.0,
            last_input: 0.0,
        };
        c.reset();
        c
    }

    pub fn reset(&mut self) {
        self.integral_term = self.ki * *self.output;
        self.last_input = *self.input;
        enforce_limit(&mut self.integral_term,
                      self.min_out, self.max_out);
    }

    fn error(&self) -> f32 {
        self.setpoint - *self.input
    }

    fn since_sample(&self) -> u32 {
        millis() - self.last_sample_time
    }

    pub fn set_tuning_params(&mut self, p: f32, i: f32, d: f32) {
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

    pub fn compute(&mut self) {
        if self.since_sample() < self.sample_time {
            return;
        } else {
            let error = self.error();

            let d_input = self.last_input - *self.input;
            self.integral_term += self.ki * error;

            enforce_limit(&mut self.integral_term,
                          self.min_out, self.max_out);

            let mut output = (self.kp * error) + 
                self.integral_term - 
                (self.kd * d_input);

            self.last_input = *self.input;
            self.last_sample_time = millis();

            enforce_limit(&mut output,
                         self.min_out, self.max_out);
            *self.output = output;
        }
    }
}
