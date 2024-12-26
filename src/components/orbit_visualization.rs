use leptos::prelude::*;
use super::planet::PlanetData;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use wasm_bindgen::{JsCast, closure::Closure};
use crate::utils::{orbital_velocity::calculate_orbital_velocity, radius::calculate_radius, gravitational_force_sun::calculate_gravitational_force_with_sun};
use std::f64::consts::PI;
use serde_wasm_bindgen::to_value;

#[derive(PartialEq, Clone)]
struct RadiusPoint {
    angle: f64,
    radius: f64
}

impl RadiusPoint {
    fn new(angle: f64, radius: f64) -> RadiusPoint {
        Self {angle, radius}
    }
}

fn get_mouse_position(event: &MouseEvent, canvas: &HtmlCanvasElement) -> (i32, i32, f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let x = event.client_x();
    let y = event.client_y();
    let canvas_x = event.client_x() as f64 - rect.left();
    let canvas_y = event.client_y() as f64 - rect.top();
    (x, y, canvas_x, canvas_y)
}

fn normalize_radius_points(points: &mut [RadiusPoint], (apoapsis, periapsis, vert_radius): (f64, f64, f64), height: f64, width: f64) -> (f64, f64, f64, f64) {
    if let Some(max_radius) = points.iter().map(|p| p.radius).max_by(|a, b| a.partial_cmp(b).unwrap()) {
        for point in points.iter_mut() {
            point.radius = (point.radius / max_radius) * height.min(width) / 2.0;
        }

        ((apoapsis / max_radius) * height.min(width) / 2.0, (periapsis / max_radius) * height.min(width) / 2.0, (vert_radius / max_radius) * height.min(width) / 2.0, max_radius)
    }
    else {
        (0.0, 0.0, 0.0, 0.0)
    }
}

fn get_radius_points(a: f64, e: f64, start: f64, end: f64, step: f64) -> Vec<RadiusPoint> {
    let mut data = Vec::new();
    let mut angle = start;
    while angle < end {
        let radius: f64 = calculate_radius(a, e, angle);
        data.push(RadiusPoint::new(angle, radius));
        angle += step;
    }
    data
}

fn draw_grid_lines(ctx: &CanvasRenderingContext2d, width: u32, height: u32, apoapsis: f64, periapsis: f64, apoapsis_normalized: f64, periapsis_normalized: f64, vert_radius_normalized: f64) {
    // Draw the grid-lines and labels
    ctx.set_stroke_style_str("white");
    ctx.set_fill_style_str("white");
    ctx.set_line_dash(&to_value(&[5, 3]).unwrap()).expect("Failed to set line dash");
    ctx.begin_path();
    ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
    ctx.line_to(width as f64 / 2.0, (height as f64 / 2.0) + vert_radius_normalized);
    ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
    ctx.line_to(width as f64 / 2.0, (height as f64 / 2.0) - vert_radius_normalized);
    ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
    ctx.line_to((width as f64 / 2.0) - apoapsis_normalized, height as f64 / 2.0);
    ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
    ctx.line_to((width as f64 / 2.0) + periapsis_normalized, height as f64 / 2.0);
    ctx.stroke();
    ctx.set_font("20px Arial");
    ctx.set_text_align("center");
    ctx.fill_text(format!("{} AU", (apoapsis / (1.496 * (10.0_f64).powi(11)) * 100.0).round() / 100.0).as_str(), (width as f64 / 2.0) - (apoapsis_normalized / 2.0), height as f64 / 2. - 5.0).expect("Failed to write text");
    ctx.fill_text(format!("{} AU", (periapsis / (1.496 * (10.0_f64).powi(11)) * 100.0).round() / 100.0).as_str(), (width as f64 / 2.0) + (periapsis_normalized / 2.0), height as f64 / 2. - 5.0).expect("Failed to write text");
    ctx.set_line_dash(&to_value::<Vec<u32>>(&vec![]).unwrap()).expect("Failed to set line dash");

}

fn draw_orbit(ctx: &CanvasRenderingContext2d, radius_points: &[RadiusPoint], width: u32, height: u32) {
    // Draw the orbit
    ctx.set_stroke_style_str("white");
    ctx.begin_path();
    for RadiusPoint { angle, radius } in radius_points {
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        ctx.line_to(x + width as f64 / 2.0, y + height as f64 / 2.0);
    }
    ctx.stroke();
}

//Canvas visualization of the orbit with eccentricity and proportionally sized sun
#[component]
pub fn OrbitVisualization(planet: ReadSignal<PlanetData>) -> impl IntoView {
    let canvas_ref = NodeRef::new();
    // Tuple of (is_hovering, angle, radius, velocity)
    let (mouse_properties, set_mouse_properties) = signal((false, 0.0, 0.0, 0.0, 0.0));

    Effect::new(move |_| {
        let planet = planet.get();
        let mut radius_points = get_radius_points(
            (planet.a).0.get(),
            (planet.e).0.get(),
            0.0,
            2.0 * PI,
            0.01,
        );

        if let Some(canvas_without_type) = canvas_ref.get() {
            let canvas = canvas_without_type as HtmlCanvasElement;
            let width = canvas.offset_width() as u32;
            let height = canvas.offset_height() as u32;

            canvas.set_width(width);
            canvas.set_height(height);


            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
                
            // Clear the canvas
            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            ctx.set_stroke_style_str("white");
            ctx.set_fill_style_str("white");

            let periapsis = calculate_radius((planet.a).0.get(), (planet.e).0.get(), 0.0);
            let apoapsis = calculate_radius((planet.a).0.get(), (planet.e).0.get(), PI);
            let vert_radius: f64 = calculate_radius((planet.a).0.get(), (planet.e).0.get(), PI / 2.0);

            let (apoapsis_normalized, periapsis_normalized, vert_radius_normalized, max_radius) = normalize_radius_points(&mut radius_points, (apoapsis, periapsis, vert_radius), width as f64, height as f64);

            draw_grid_lines(&ctx, width, height, apoapsis, periapsis, apoapsis_normalized, periapsis_normalized, vert_radius_normalized);

            draw_orbit(&ctx, &radius_points, width, height);

            // Add mousemove event listener
            let canvas_clone = canvas.clone();
            let hover_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let (x, y, canvas_x, canvas_y) = get_mouse_position(&event, &canvas_clone);

                if canvas_x >= 0.0 && canvas_x <= width as f64 && canvas_y >= 0.0 && canvas_y <= height as f64 {
                    let canvas_x = canvas_x - width as f64 / 2.0;
                    let canvas_y = canvas_y - height as f64 / 2.0;
                    let angle = canvas_y.atan2(canvas_x);
                    let radius = calculate_radius(planet.a.0.get_untracked(), planet.e.0.get_untracked(), angle);
                    let velocity = calculate_orbital_velocity(planet.a.0.get_untracked(), planet.e.0.get_untracked(), planet.m_sun, planet.g, angle);
                    let gravitational_force = calculate_gravitational_force_with_sun(planet.m_object.0.get_untracked(), planet.m_sun, planet.m_earth, planet.a.0.get_untracked(), planet.e.0.get_untracked(), planet.g, angle);
                    set_mouse_properties((true, angle, radius, velocity, gravitational_force));

                    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                    draw_orbit(&ctx, &radius_points, width, height);        

                    let radius_normalized: f64 = (radius / max_radius) * (height.min(width) as f64) / 2.0;
                    ctx.set_stroke_style_str("red");
                    ctx.begin_path();
                    ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
                    ctx.line_to(radius_normalized * angle.cos() + width as f64 / 2.0, radius_normalized * angle.sin() + height as f64 / 2.0);
                    ctx.stroke();

                    draw_grid_lines(&ctx, width, height, apoapsis, periapsis, apoapsis_normalized, periapsis_normalized, vert_radius_normalized);

                    let info_div = document().get_element_by_id("orbit_visualization_info").unwrap();
                    info_div.set_attribute("style", &format!("left: {}px; top: {}px;", x, y)).unwrap();
                } else {
                    set_mouse_properties((false, 0.0, 0.0, 0.0, 0.0));
                }
            }) as Box<dyn FnMut(_)>);
            document().add_event_listener_with_callback("mousemove", hover_closure.as_ref().unchecked_ref()).unwrap();
            hover_closure.forget();
        }
    });

    view! { 
        <canvas id="orbit_visualization_canvas" node_ref=canvas_ref/>
        <Show when=move || { mouse_properties.get().0 }>
            <div id="orbit_visualization_info">
                <Show when=move || {mouse_properties.get().4 != 0.0}
                    fallback=move || {view!{
                        <span>{format!("At angle: {:.2} Rads and radius {:.2} AU the velocity is {:.3} km/s", mouse_properties.get().1,  mouse_properties.get().2 / (1.496 * (10.0_f64).powi(11)), mouse_properties.get().3)}</span>
                    }}>
                    <span>{format!("At angle: {:.2} Rads and radius {:.2} AU the velocity is {:.3} km/s and the gravitational force with the Sun is {:.3e} N", mouse_properties.get().1,  mouse_properties.get().2 / (1.496 * (10.0_f64).powi(11)), mouse_properties.get().3, mouse_properties.get().4)}</span>
                </Show>
            </div>
        </Show>
    }
}