use core::time::Duration;

pub struct RelativeDuration<D = Duration> {
    duration: D,
    sign: i8,
}

impl RelativeDuration {
    pub fn from_secs_f32(secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(secs.abs()),
            sign: secs.signum() as i8,
        }
    }

    pub fn from_secs_f64(secs: f64) -> Self {
        Self {
            duration: Duration::from_secs_f64(secs.abs()),
            sign: secs.signum() as i8,
        }
    }

    pub const fn from_secs(secs: u64) -> Self {
        Self {
            duration: Duration::from_secs(secs),
            sign: 1,
        }
    }

    pub const fn as_secs_f32(&self) -> f32 {
        self.duration.as_secs_f32() * self.sign as f32
    }
}
