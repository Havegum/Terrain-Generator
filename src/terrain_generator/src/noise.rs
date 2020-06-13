use bracket_noise::prelude::*;
use bracket_random::prelude::*;

pub struct Noise {
    height: FastNoise,
    theta: FastNoise,
    offset: FastNoise,
    // noise_resources: FastNoise,
    uniform: RandomNumberGenerator,
}

impl Noise {
    pub fn new (seed: u64) -> Noise {
        let mut height = FastNoise::seeded(seed);
        height.set_noise_type(NoiseType::SimplexFractal);
        height.set_fractal_type(FractalType::FBM);
        height.set_fractal_octaves(5);
        height.set_fractal_gain(0.6);
        height.set_fractal_lacunarity(3.0);
        height.set_frequency(0.85);

        let mut theta = FastNoise::seeded(seed + 1);
        theta.set_noise_type(NoiseType::Simplex);
        theta.set_frequency(2.0);

        let mut offset = FastNoise::seeded(seed + 2);
        offset.set_noise_type(NoiseType::Simplex);
        offset.set_frequency(2.0);

        // let mut noise_resources = FastNoise::seeded(seed + 3);
        // noise_resources.set_noise_type(NoiseType::Simplex);
        // noise_resources.set_frequency(2.0);

        let uniform = RandomNumberGenerator::seeded(seed);

        Noise {
            height,
            theta,
            offset,
            // noise_resources,
            uniform,
        }
    }

    pub fn height (&self, x: f64, y: f64) -> f64 {
        self.height.get_noise(x as f32, y as f32) as f64
    }

    pub fn theta (&self, x: f64, y: f64) -> f64 {
        self.theta.get_noise(x as f32, y as f32) as f64
    }

    pub fn offset (&self, x: f64, y: f64) -> f64 {
        self.offset.get_noise(x as f32, y as f32) as f64
    }

    pub fn rng (&mut self) -> f64 {
        self.uniform.rand::<f64>()
    }
}
