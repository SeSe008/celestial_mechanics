//use crate::components::calculations::ExampleSedna;
use leptos::prelude::*;

use crate::components::{planet::{Inputs, create_planet}, orbital_velocity_chart::VelocityChart, orbit_visualization::OrbitVisualization, small_properties::SmallProperties, gravitational_force_with_sun_chart::GravitationalForceWithSunChart, orbit_visualization_options::OrbitVisualizationOptions};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    //Default values for Sedna
    let (planet, update_planet) = signal(create_planet(1.0, 0.017, 1.0, 1.0));

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

        <div class="container">
            <OrbitVisualization planet={planet}/>
            <VelocityChart planet={planet}/>
            <SmallProperties planet={planet} />
            <Inputs planet_signal=(planet, update_planet)/>
            <GravitationalForceWithSunChart planet={planet} />
            <OrbitVisualizationOptions planet={planet} />
        </div>
        </ErrorBoundary>
    }
}
