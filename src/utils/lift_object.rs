//Calculate how much energy is needed to lift a 1000kg object 75km high
pub fn calculate_lift_energy(d_ratio_earth: f64, d_earth: f64, m_planet_ratio_earth: f64, m_earth: f64, g: f64) -> f64 {
    let d = d_ratio_earth * d_earth * 1000.0;
    let m_planet = m_planet_ratio_earth * m_earth;
    let m_object = 1000.0;
    let height = 75000.0;

    let r_initial = d / 2.0;
    let r_final = r_initial + height;

    g * m_object * m_planet * (1.0 / r_initial - 1.0 / r_final)
}