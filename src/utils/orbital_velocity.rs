use crate::utils::radius::calculate_radius;

// Calculate the orbital velocity of a celestial object
pub fn calculate_orbital_velocity(a: f64, e: f64, m_sun: f64, g_constant: f64, angle: f64) -> f64 {
    f64::sqrt(g_constant * m_sun * (2.0 / calculate_radius(a, e, angle) - (1.0 / a))).round() / 1000.0
}