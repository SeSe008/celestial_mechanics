// Calculate the gravitational acceleration on a celestial object at surface level
pub fn calculate_gravitational_acceleration(d: f64, m: f64, m_earth: f64) -> f64 {
    // Convert Earth masses to kilograms and Earth radii to meters
    let mass_kg = m * m_earth; // Earth mass in kg
    let radius_m = (d * 12742.46 * 1000.0) / 2.0; // Earth-ratio-diameter in meters, then radius

    (6.6743 * 10.0_f64.powi(-11) * mass_kg) / radius_m.powi(2)
}