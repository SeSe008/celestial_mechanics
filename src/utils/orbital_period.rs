use std::f64::consts::PI;

//Calculate the orbital period of a planet
pub fn calculate_orbital_period(a: f64, g: f64, m_object_earth_ratio: f64, m_sun: f64, m_earth: f64) -> f64 {
    let m_object_kg = m_object_earth_ratio * m_earth;
    let period_seconds = 2.0 * PI * f64::sqrt(a.powi(3) / (g * (m_object_kg + m_sun)));
    period_seconds / 60.0 / 60.0 / 24.0 / 365.25 // Convert seconds to years
}