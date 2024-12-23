use leptos::prelude::*;

use crate::utils::{escape_velocity::calculate_escape_velocity, gravitational_acceleration::calculate_gravitational_acceleration, orbital_period::calculate_orbital_period};
use super::planet::PlanetData;


#[component]
fn EscapeVelocity(planet: ReadSignal<PlanetData>) -> impl IntoView {
    view! {
        <div class="small_property">
            <span>"Escape Velocity: " {move || format!("{:.3}", calculate_escape_velocity(planet.get().d.0.get(), planet.get().m_object.0.get(), planet.get().m_earth))} "km/s"</span>
        </div>
    }
}

#[component]
fn GravitationalAcceleration(planet: ReadSignal<PlanetData>) -> impl IntoView {
    view! {
        <div class="small_property">
            <span>"Gravitational Acceleration: " {move || format!("{:.4}", calculate_gravitational_acceleration(planet.get().d.0.get(), planet.get().m_object.0.get(), planet.get().m_earth))} " m/(s²)"</span>
        </div>
    }
}

#[component]
fn RotationalPeriod(planet: ReadSignal<PlanetData>) -> impl IntoView {
    view! {
        <div class="small_property">
            <span>"Orbital Period: " {move || format!("{:.2}", calculate_orbital_period(planet.get().a.0.get(), planet.get().g, planet.get().m_object.0.get(), planet.get().m_sun, planet.get().m_earth))} " years"</span>
        </div>
    }
}

#[component]
pub fn SmallProperties(planet: ReadSignal<PlanetData>) -> impl IntoView {
    view! {
        <div id="small_properties">
            <Show when=move || {planet.get().m_object.0.get() != 0.0}>
                <EscapeVelocity planet={planet} />
                <GravitationalAcceleration planet={planet} />
            </Show>
            <RotationalPeriod planet={planet} />
        </div>
    }
}