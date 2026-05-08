use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectralClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl SpectralClass {
    pub const ALL: [SpectralClass; 7] = [
        SpectralClass::O,
        SpectralClass::B,
        SpectralClass::A,
        SpectralClass::F,
        SpectralClass::G,
        SpectralClass::K,
        SpectralClass::M,
    ];

    pub fn name(&self) -> &str {
        match self {
            SpectralClass::O => "O",
            SpectralClass::B => "B",
            SpectralClass::A => "A",
            SpectralClass::F => "F",
            SpectralClass::G => "G",
            SpectralClass::K => "K",
            SpectralClass::M => "M",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SpectralClass::O => "Blue",
            SpectralClass::B => "Blue-white",
            SpectralClass::A => "White",
            SpectralClass::F => "Yellow-white",
            SpectralClass::G => "Yellow",
            SpectralClass::K => "Orange",
            SpectralClass::M => "Red",
        }
    }

    pub fn temp_range(&self) -> (f64, f64) {
        match self {
            SpectralClass::O => (30000.0, 60000.0),
            SpectralClass::B => (10000.0, 30000.0),
            SpectralClass::A => (7500.0, 10000.0),
            SpectralClass::F => (6000.0, 7500.0),
            SpectralClass::G => (5200.0, 6000.0),
            SpectralClass::K => (3700.0, 5200.0),
            SpectralClass::M => (2400.0, 3700.0),
        }
    }

    pub fn mass_range(&self) -> (f64, f64) {
        match self {
            SpectralClass::O => (16.0, 150.0),
            SpectralClass::B => (2.0, 16.0),
            SpectralClass::A => (1.4, 2.1),
            SpectralClass::F => (1.04, 1.4),
            SpectralClass::G => (0.8, 1.04),
            SpectralClass::K => (0.45, 0.8),
            SpectralClass::M => (0.08, 0.45),
        }
    }

    pub fn weights() -> [f64; 7] {
        [0.00003, 0.13, 0.6, 3.0, 7.6, 12.1, 76.45]
    }

    pub fn sample<R: Rng>(rng: &mut R) -> Self {
        let weights = Self::weights();
        let dist = WeightedIndex::new(&weights).unwrap();
        match dist.sample(rng) {
            0 => SpectralClass::O,
            1 => SpectralClass::B,
            2 => SpectralClass::A,
            3 => SpectralClass::F,
            4 => SpectralClass::G,
            5 => SpectralClass::K,
            _ => SpectralClass::M,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitableZone {
    pub inner_au: f64,
    pub outer_au: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub id: usize,
    pub class: SpectralClass,
    pub temperature_k: f64,
    pub luminosity_solar: f64,
    pub mass_solar: f64,
    pub position: [f32; 2],
    pub color: [u8; 4],
    pub habitable_zone: Option<HabitableZone>,
}

impl Star {
    pub fn name(&self) -> String {
        format!("HD-{:06}", self.id)
    }

    pub fn screen_radius(&self) -> f32 {
        let norm = ((self.luminosity_solar.log10() + 3.0) / 8.0).clamp(0.0, 1.0) as f32;
        1.0 + 2.0 * norm
    }
}
