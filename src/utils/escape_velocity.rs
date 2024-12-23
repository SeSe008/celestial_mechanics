use crate::utils::gravitational_acceleration::calculate_gravitational_acceleration;

// Calculate the escape velocity of a planet
pub fn calculate_escape_velocity(d: f64, m: f64, m_earth: f64) -> f64 {
    // Convert Earth radii to meters
    let radius_m = (d * 12742.46 * 1000.0) / 2.0; // Earth-ratio-diameter in meters, then to radius

    (f64::sqrt(2.0 * calculate_gravitational_acceleration(d, m, m_earth) * radius_m)) / 1000.0
} 