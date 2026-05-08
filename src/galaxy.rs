use rand::{Rng, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::star::{SpectralClass, Star};
use crate::color;
use crate::habitable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    pub stars: Vec<Star>,
    pub seed: u64,
    pub star_count: usize,
}

impl Galaxy {
    pub fn generate(seed: u64, star_count: usize) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        let arm_count = (star_count as f64 * 0.70) as usize;
        let core_count = (star_count as f64 * 0.20) as usize;
        let scatter_count = star_count.saturating_sub(arm_count + core_count);

        const GALAXY_RADIUS: f64 = 5000.0;
        const N_ARMS: usize = 4;
        const ARM_TIGHTNESS: f64 = 0.4;

        let mut stars = Vec::with_capacity(star_count);
        let mut id = 0usize;

        // Spiral arms
        for i in 0..arm_count {
            let arm = i % N_ARMS;
            let theta_offset = arm as f64 * 2.0 * PI / N_ARMS as f64;
            let theta = rng.gen_range(0.0..4.0 * PI);
            let r = 150.0 * (ARM_TIGHTNESS * theta).exp();
            let scatter_sigma = r * 0.12;
            let scatter = Normal::new(0.0, scatter_sigma).unwrap().sample(&mut rng);
            let r_final = (r + scatter.abs()).min(GALAXY_RADIUS);
            let angle = theta + theta_offset;
            let x = (r_final * angle.cos()) as f32;
            let y = (r_final * angle.sin()) as f32;
            stars.push(generate_star_at(id, &mut rng, [x, y]));
            id += 1;
        }

        // Core bulge
        let core_sigma = 600.0;
        for _ in 0..core_count {
            let x = Normal::new(0.0, core_sigma).unwrap().sample(&mut rng) as f32;
            let y = Normal::new(0.0, core_sigma).unwrap().sample(&mut rng) as f32;
            stars.push(generate_star_at(id, &mut rng, [x, y]));
            id += 1;
        }

        // Scattered disk
        for _ in 0..scatter_count {
            let r = rng.gen_range(0.0..GALAXY_RADIUS);
            let angle = rng.gen_range(0.0..2.0 * PI);
            let x = (r * angle.cos()) as f32;
            let y = (r * angle.sin()) as f32;
            stars.push(generate_star_at(id, &mut rng, [x, y]));
            id += 1;
        }

        Galaxy {
            stars,
            seed,
            star_count,
        }
    }
}

fn generate_star_at(id: usize, rng: &mut StdRng, position: [f32; 2]) -> Star {
    let class = SpectralClass::sample(rng);
    let (t_min, t_max) = class.temp_range();
    let temperature_k = rng.gen_range(t_min..t_max);
    let (m_min, m_max) = class.mass_range();
    let mass_solar = rng.gen_range(m_min..m_max);
    let luminosity_solar = mass_to_luminosity(mass_solar);
    let star_color = color::kelvin_to_rgba(temperature_k);
    let habitable_zone = habitable::habitable_zone(luminosity_solar);
    Star {
        id,
        class,
        temperature_k,
        luminosity_solar,
        mass_solar,
        position,
        color: star_color,
        habitable_zone,
    }
}

fn mass_to_luminosity(m: f64) -> f64 {
    if m < 0.43 {
        0.23 * m.powf(2.3)
    } else if m <= 2.0 {
        m.powf(4.0)
    } else if m <= 55.0 {
        1.4 * m.powf(3.5)
    } else {
        32000.0 * m
    }
}
