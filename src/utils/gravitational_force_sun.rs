use crate::utils::radius::calculate_radius;

//Calculate the gravitational force between a object and the sun at a given angle
pub fn calculate_gravitational_force_with_sun(m_object_earth_ratio: f64, m_sun: f64, m_earth: f64, a: f64, e: f64, g: f64, angle: f64) -> f64{
    let m_object_kg = m_object_earth_ratio * m_earth;

    g * m_object_kg * m_sun / calculate_radius(a, e, angle).powi(2)
}