use crate::star::HabitableZone;

pub fn habitable_zone(luminosity_solar: f64) -> Option<HabitableZone> {
    if luminosity_solar < 0.0001 || luminosity_solar > 100_000.0 {
        return None;
    }
    let sqrt_l = luminosity_solar.sqrt();
    Some(HabitableZone {
        inner_au: 0.95 * sqrt_l,
        outer_au: 1.37 * sqrt_l,
    })
}
