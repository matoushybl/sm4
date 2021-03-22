use crate::float;
use embedded_time::duration::Microseconds;

#[derive(Copy, Clone)]
pub struct PSDController {
    sum: f32,
    previous: f32,
    sampling_period: f32, // seconds
}

impl PSDController {
    pub fn new(sampling_period: Microseconds) -> Self {
        Self {
            sum: 0.0,
            previous: 0.0,
            sampling_period: sampling_period.0 as f32 / 1_000_000.0,
        }
    }

    pub fn sample(
        &mut self,
        desired: &f32,
        actual: &f32,
        p: &f32,
        s: &f32,
        d: &f32,
        limit: &f32,
    ) -> f32 {
        let e = desired - actual;

        self.sum += e * self.sampling_period;
        self.sum = float::fmaxf(float::fminf(self.sum, *limit), -*limit);

        let x = e * p + d * (e - self.previous) / self.sampling_period + s * self.sum;
        self.previous = e;

        float::fmaxf(float::fminf(x, *limit), -*limit)
    }
}
