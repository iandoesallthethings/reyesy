pub trait F32Extension {
    fn times(&self, other: f32) -> f32;
    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32;
    fn normalize(&self, input_min: f32, input_max: f32) -> f32;
    fn denormalize(&self, output_min: f32, output_max: f32) -> f32;
}

impl F32Extension for f32 {
    fn times(&self, other: f32) -> f32 {
        self * other
    }

    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32 {
        // Great explanation of this algorithm here:
        // https://stats.stackexchange.com/questions/281162/scale-a-number-between-a-range/281164
        let input_size = input_max - input_min;
        let normalized = (self - input_min) / input_size;

        let output_size = output_max - output_min;
        (normalized * output_size) + output_min
    }

    fn normalize(&self, input_min: f32, input_max: f32) -> f32 {
        self.rescale(input_min, input_max, 0.0, 1.0)
    }

    fn denormalize(&self, output_min: f32, output_max: f32) -> f32 {
        self.rescale(0.0, 1.0, output_min, output_max)
    }
}
