// Calculate the radius for a point in an elliptical orbit
pub fn calculate_radius(a: f64, e: f64, angle: f64) -> f64 {
    (a * (1.0 - e * e))/(1.0 + e * (angle as f64).cos())
}