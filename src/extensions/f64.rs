pub trait F64Extension {
    fn times(&self, other: f64) -> f64;
    fn rescale(&self, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64;
    fn normalize(&self, input_min: f64, input_max: f64) -> f64;
    fn denormalize(&self, output_min: f64, output_max: f64) -> f64;
}

impl F64Extension for f64 {
    fn times(&self, other: f64) -> f64 {
        self * other
    }

    fn rescale(&self, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64 {
        // Great explanation of this algorithm here:
        // https://stats.stackexchange.com/questions/281162/scale-a-number-between-a-range/281164
        let input_size = input_max - input_min;
        let normalized = (self - input_min) / input_size;

        let output_size = output_max - output_min;
        (normalized * output_size) + output_min
    }

    fn normalize(&self, input_min: f64, input_max: f64) -> f64 {
        self.rescale(input_min, input_max, 0.0, 1.0)
    }

    fn denormalize(&self, output_min: f64, output_max: f64) -> f64 {
        self.rescale(0.0, 1.0, output_min, output_max)
    }
}
