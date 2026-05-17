use core::time::Duration;

use crate::time::RelativeDuration;

pub trait FromDurationAsSecs<D = Duration> {
    fn from_duration_as_secs(duration: D) -> Self;
}

impl FromDurationAsSecs for f32 {
    fn from_duration_as_secs(duration: Duration) -> Self {
        duration.as_secs_f32()
    }
}

impl FromDurationAsSecs for f64 {
    fn from_duration_as_secs(duration: Duration) -> Self {
        duration.as_secs_f64()
    }
}

pub trait FromSecs<T> {
    fn from_secs(secs: T) -> Self;
}

impl FromSecs<f32> for Duration {
    fn from_secs(secs: f32) -> Self {
        Duration::from_secs_f32(secs)
    }
}

impl FromSecs<f64> for Duration {
    fn from_secs(secs: f64) -> Self {
        Duration::from_secs_f64(secs)
    }
}

impl FromSecs<f32> for RelativeDuration {
    fn from_secs(secs: f32) -> Self {
        RelativeDuration::from_secs_f32(secs)
    }
}

impl FromSecs<f64> for RelativeDuration {
    fn from_secs(secs: f64) -> Self {
        RelativeDuration::from_secs_f64(secs)
    }
}
