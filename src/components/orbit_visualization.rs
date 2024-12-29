use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use super::planet::PlanetData;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use wasm_bindgen::{JsCast, closure::Closure};
use crate::utils::{gravitational_force_sun::calculate_gravitational_force_with_sun, orbital_velocity::calculate_orbital_velocity, orbits::load_orbits, radius::calculate_radius};
use std::{f64::consts::PI, rc::Rc, cell::RefCell};
use serde_wasm_bindgen::to_value;

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub struct RadiusPoint {
    angle: f64,
    pub radius: f64
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

fn normalize_radius_points(radius_points_vec: &mut Vec<(i8, Vec<RadiusPoint>)>, radius_points: &mut [RadiusPoint], height: f64, width: f64) -> f64 {

    let max_radius = radius_points
        .iter()
        .map(|p| p.radius)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if let Some(max_radius) = max_radius {
        for section in radius_points_vec {
            for point in section.1.iter_mut() {
                point.radius = (point.radius / max_radius) * height.min(width) / 2.0;
            }
        }
        for point in radius_points {
            point.radius = (point.radius / max_radius) * height.min(width) / 2.0;
        }

        max_radius
    } else {
        0.0
    }
}


pub fn get_radius_points(a: f64, e: f64, start: f64, end: f64, step: f64) -> Vec<RadiusPoint> {
    let mut data = Vec::new();
    let mut angle = start;
    while angle < end {
        let radius: f64 = calculate_radius(a, e, angle);
        data.push(RadiusPoint::new(angle, radius));
        angle += step;
    }
    data
}

fn mouse_hover(ctx: Rc<CanvasRenderingContext2d>, width: f64, height: f64, max_radius: f64, canvas: Rc<HtmlCanvasElement>, planet: Rc<RefCell<PlanetData>>, set_mouse_properties: WriteSignal<(bool, f64, f64, f64, f64)>) {
    // Add mousemove event listener
    let hover_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let (x, y, canvas_x, canvas_y) = get_mouse_position(&event, &canvas);

        if canvas_x >= 0.0 && canvas_x <= width as f64 && canvas_y >= 0.0 && canvas_y <= height as f64 {
            let planet = planet.borrow();

            let canvas_x = canvas_x - width as f64 / 2.0;
            let canvas_y = canvas_y - height as f64 / 2.0;
            let angle = canvas_y.atan2(canvas_x);                   
            let radius = calculate_radius(planet.a.0.get_untracked(), planet.e.0.get_untracked(), angle);
            let velocity = calculate_orbital_velocity(planet.a.0.get_untracked(), planet.e.0.get_untracked(), planet.m_sun, planet.g, angle);
            let gravitational_force = calculate_gravitational_force_with_sun(planet.m_object.0.get_untracked(), planet.m_sun, planet.m_earth, planet.a.0.get_untracked(), planet.e.0.get_untracked(), planet.g, angle);
            set_mouse_properties((true, angle, radius, velocity, gravitational_force));

            draw_scene(*planet, set_mouse_properties, false);

            let radius_normalized: f64 = (radius / max_radius) * (height.min(width) as f64) / 2.0;
            ctx.set_stroke_style_str("red");    
            ctx.begin_path();
            ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
            ctx.line_to(radius_normalized * angle.cos() + width as f64 / 2.0, radius_normalized * angle.sin() + height as f64 / 2.0);
            ctx.stroke();

            let info_div = document().get_element_by_id("orbit_visualization_info").unwrap();
            info_div.set_attribute("style", &format!("left: {}px; top: {}px;", x, y)).unwrap();
        } else {
            set_mouse_properties((false, 0.0, 0.0, 0.0, 0.0));
        }
    }) as Box<dyn FnMut(_)>);
    document().add_event_listener_with_callback("mousemove", hover_closure.as_ref().unchecked_ref()).unwrap();
    hover_closure.forget();
}

fn draw_grid_lines(ctx: &CanvasRenderingContext2d, width: f64, height: f64, planet: PlanetData, max_radius: f64) {

    //Calculate special points
    let periapsis = calculate_radius((planet.a).0.get(), (planet.e).0.get(), 0.0);
    let periapsis_normalized = (periapsis / max_radius) * height.min(width) / 2.0;
    let apoapsis = calculate_radius((planet.a).0.get(), (planet.e).0.get(), PI);
    let apoapsis_normalized = (apoapsis / max_radius) * height.min(width) / 2.0;
    let vert_radius = calculate_radius((planet.a).0.get(), (planet.e).0.get(), PI / 2.0);
    let vert_radius_normalized = (vert_radius / max_radius) * height.min(width) / 2.0;

    
    // Draw the grid-lines and labels
    ctx.set_stroke_style_str("white");
    ctx.set_fill_style_str("white");
    ctx.set_line_dash(&to_value(&[5, 3]).unwrap()).expect("Failed to set line dash");
    ctx.begin_path();
    ctx.move_to(width / 2.0, height / 2.0);
    ctx.line_to(width / 2.0, (height / 2.0) + vert_radius_normalized);
    ctx.move_to(width / 2.0, height / 2.0);
    ctx.line_to(width / 2.0, (height / 2.0) - vert_radius_normalized);
    ctx.move_to(width / 2.0, height / 2.0);
    ctx.line_to((width / 2.0) - apoapsis_normalized, height / 2.0);
    ctx.move_to(width / 2.0, height / 2.0);
    ctx.line_to((width / 2.0) + periapsis_normalized, height / 2.0);
    ctx.stroke();
    ctx.set_font("20px Arial");
    ctx.set_text_align("center");
    ctx.fill_text(format!("{} AU", (apoapsis / (1.496 * (10.0_f64).powi(11)) * 100.0).round() / 100.0).as_str(), (width / 2.0) - (apoapsis_normalized / 2.0), height / 2. - 5.0).expect("Failed to write text");
    ctx.fill_text(format!("{} AU", (periapsis / (1.496 * (10.0_f64).powi(11)) * 100.0).round() / 100.0).as_str(), (width / 2.0) + (periapsis_normalized / 2.0), height / 2. - 5.0).expect("Failed to write text");
    ctx.set_line_dash(&to_value::<Vec<u32>>(&vec![]).unwrap()).expect("Failed to set line dash");

}

fn draw_stored_orbits(ctx: &CanvasRenderingContext2d, radius_points_vec: Vec<(i8, Vec<RadiusPoint>)>, width: f64, height: f64) {
    ctx.set_stroke_style_str("lime");
    for i in 0..radius_points_vec.len() {
        ctx.begin_path();
        for RadiusPoint { angle, radius } in &radius_points_vec[i].1 {
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            ctx.line_to(x + width / 2.0, y + height / 2.0);      
        }
        ctx.stroke();
    }
}

fn draw_orbit(ctx: &CanvasRenderingContext2d, radius_points: &[RadiusPoint], width: f64, height: f64) {
    // Draw the orbit
    ctx.set_stroke_style_str("white");
    ctx.begin_path();
    for RadiusPoint { angle, radius } in radius_points {
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        ctx.line_to(x + width / 2.0, y + height / 2.0);
    }
    ctx.stroke();
}

pub fn draw_scene(planet: PlanetData, set_mouse_properties: WriteSignal<(bool, f64, f64, f64, f64)>, with_hover: bool) {
    let mut radius_points = get_radius_points(
        (planet.a).0.get(),
        (planet.e).0.get(),
        0.0,
        2.0 * PI,
        0.01,
    );

    let canvas = document().
        get_element_by_id("orbit_visualization_canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let width = canvas.offset_width() as f64;
    let height = canvas.offset_height() as f64;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.clear_rect(0.0, 0.0, width, height);

    let mut stored_radius_points = load_orbits();

    let max_radius = normalize_radius_points(&mut stored_radius_points, &mut radius_points, width, height);

    // Draw stored orbits
    draw_stored_orbits(&ctx, stored_radius_points, width, height);

    // Draw the orbit
    draw_orbit(&ctx, &radius_points, width, height);

    // Draw grid lines
    draw_grid_lines(&ctx, width, height, planet, max_radius);

    if with_hover { 
        mouse_hover(Rc::new(ctx), width, height, max_radius, Rc::new(canvas), Rc::new(RefCell::new(planet)), set_mouse_properties);
    }
}

pub fn create_scene(planet_signal: ReadSignal<PlanetData>, set_mouse_properties: WriteSignal<(bool, f64, f64, f64, f64)>) {
    Effect::new(move |_| {
        let planet = planet_signal.get();

        draw_scene(planet, set_mouse_properties, true);
    });
}

//Canvas visualization of the orbit with eccentricity and labels
#[component]
pub fn OrbitVisualization(planet: ReadSignal<PlanetData>) -> impl IntoView {
    // Tuple of (is_hovering, angle, radius, velocity)
    let (mouse_properties, set_mouse_properties) = signal((false, 0.0, 0.0, 0.0, 0.0));
    
    create_scene(planet, set_mouse_properties);

    view! { 
        <canvas id="orbit_visualization_canvas" />
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