use leptos::prelude::*;

use crate::{components::orbit_visualization::{get_radius_points, draw_scene}, utils::orbits::{add_orbit, remove_last_orbit, clear_orbits}};
use super::planet::PlanetData;

#[component]
pub fn OrbitVisualizationOptions(planet: ReadSignal<PlanetData>) -> impl IntoView {
    view!{
        <div id="orbit_visualization_options">
            <span>"Add or remove orbits to the visualization"</span>
            <button on:click=move |_| {
                let radius_points = get_radius_points(
                    planet.get().a.0.get(),
                    planet.get().e.0.get(),
                    0.0,
                    2.0 * std::f64::consts::PI,
                    0.01,
                );

                add_orbit(&radius_points);
                draw_scene(planet.get(), signal((false, 0.0, 0.0, 0.0, 0.0)).1, false, std::rc::Rc::new(std::cell::RefCell::new(None)));
            }>"Add Orbit"</button>
            <button on:click=move |_| {
                remove_last_orbit();

                draw_scene(planet.get(), signal((false, 0.0, 0.0, 0.0, 0.0)).1, false, std::rc::Rc::new(std::cell::RefCell::new(None)));
            }>"Remove last Orbit"</button>
            <button on:click=move |_| {
                clear_orbits();

                draw_scene(planet.get(), signal((false, 0.0, 0.0, 0.0, 0.0)).1, false, std::rc::Rc::new(std::cell::RefCell::new(None)));
            }>"Clear Orbits"</button>
        </div>
    }
}