#[derive(Default)]
pub struct ControllerConfig {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub kf: f64,

    pub output_min: f64,
    pub output_max: f64,
    pub integral_min: f64,
    pub integral_max: f64,

    pub deadband: f64,
    pub anti_windup: bool,
}

impl ControllerConfig {
    pub fn new() -> Self {
        Self {
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            kf: 0.0,
            output_min: f64::NEG_INFINITY,
            output_max: f64::INFINITY,
            integral_min: f64::NEG_INFINITY,
            integral_max: f64::INFINITY,
            deadband: 0.0,
            anti_windup: true,
        }
    }
    pub fn pid_controller(mut self, p: f64, i: f64, d: f64) -> Self {
        self.with_kp(p).with_ki(i).with_kd(d)
    }
    pub fn pidf_controller(mut self, p: f64, i: f64, d: f64, f: f64) -> Self {
        self.with_kp(p).with_ki(i).with_kd(d).with_kf(f)
    }
    pub fn with_kp(mut self, kp: f64) -> Self {
        self.kp = kp;
        self
    }

    pub fn with_ki(mut self, ki: f64) -> Self {
        self.ki = ki;
        self
    }

    pub fn with_kd(mut self, kd: f64) -> Self {
        self.kd = kd;
        self
    }

    pub fn with_kf(mut self, kf: f64) -> Self {
        self.kf = kf;
        self
    }
    pub fn with_output_limits(mut self, min: f64, max: f64) -> Self {
        self.output_min = min;
        self.output_max = max;
        self
    }

    pub fn with_integral_limits(mut self, min: f64, max: f64) -> Self {
        self.integral_min = min;
        self.integral_max = max;
        self
    }
    pub fn controller_type(&self) -> String {
        let mut terms = Vec::new();
        if self.kp != 0.0 {
            terms.push("P");
        }
        if self.ki != 0.0 {
            terms.push("I");
        }
        if self.kd != 0.0 {
            terms.push("D");
        }
        if self.kf != 0.0 {
            terms.push("F");
        }

        if terms.is_empty() {
            "Disabled".to_string()
        } else {
            terms.join("")
        }
    }
}

pub struct PID {
    // Configuration
    pub config: ControllerConfig,

    // Internal state
    pub integral: f64,
    pub prev_error: f64,
    pub initialized: bool,

    // Statistics
    pub compute_count: u64,
}
impl PID {
    pub fn new(config: ControllerConfig) -> Self {
        Self {
            config,
            integral: 0.0,
            prev_error: 0.0,
            initialized: false,
            compute_count: 0,
        }
    }
    fn apply_deadband(&self, error: f64) -> f64 {
        if self.config.deadband != 0.0 {
            if error.abs() <= self.config.deadband {
                0.0
            } else if error > 0.0 {
                error - self.config.deadband
            } else {
                error + self.config.deadband
            }
        } else {
            error
        }
    }
    pub fn compute(&mut self, setpoint: f64, process_variable: f64, dt: f64) -> f64 {
        if dt <= 0.0 {
            return 0.0;
        }

        let error = self.apply_deadband(setpoint - process_variable);
        // P
        let p_term = self.config.kp * error;
        // I
        self.integral += error * dt;

        if self.config.anti_windup {
            self.integral = self
                .integral
                .clamp(self.config.integral_min, self.config.integral_max);
        }

        let i_term = self.config.ki * self.integral;

        // D
        let d_term = if self.initialized {
            self.config.kd * (error - self.prev_error) / dt
        } else {
            0.0
        };

        // F
        let f_term = self.config.kf * setpoint;

        let output = p_term + i_term + d_term + f_term;

        let clamped_output = output.clamp(self.config.output_min, self.config.output_max);

        if self.config.anti_windup && output != clamped_output {
            let excess = output - clamped_output;
            if self.config.ki != 0.0 {
                let integral_reduction = excess / self.config.ki;
                self.integral -= integral_reduction;
                self.integral = self
                    .integral
                    .clamp(self.config.integral_min, self.config.integral_max);
            }
        }

        self.prev_error = error;
        self.initialized = true;
        self.compute_count += 1;

        clamped_output
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
        self.initialized = false;
        self.compute_count = 0;
    }
    pub fn apply_ziegler_nichols_pi(&mut self, ultimate_gain: f64, ultimate_period: f64) {
        self.config.kp = 0.45 * ultimate_gain;
        self.config.ki = 0.54 * ultimate_gain / ultimate_period;
        self.config.kd = 0.0;
    }

    pub fn apply_ziegler_nichols_pid(&mut self, ultimate_gain: f64, ultimate_period: f64) {
        self.config.kp = 0.6 * ultimate_gain;
        self.config.ki = 1.2 * ultimate_gain / ultimate_period;
        self.config.kd = 0.075 * ultimate_gain * ultimate_period;
    }
}
