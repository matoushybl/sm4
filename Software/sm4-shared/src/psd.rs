#[derive(Copy, Clone, Default)]
pub struct PSDController {
    sum: f32,
    previous: f32,
}

impl PSDController {
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

        self.sum += e;

        let x = e * p + d * (actual - self.previous) + s * self.sum;
        self.previous = *actual;

        // TODO add antiwindup
        x
    }
}
