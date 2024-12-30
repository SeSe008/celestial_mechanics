use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::orbit_visualization::draw_scene;
use super::planet::PlanetData;

fn switch_view(left_right: bool, planet: PlanetData, set_mouse_properties: WriteSignal<(bool, f64, f64, f64, f64)>) {
    let container_ids = ["orbit_visualization_canvas", "velocity_chart", "gravitational_force_with_sun_chart", "small_properties"];

    let current_active = document().get_elements_by_class_name("visible_element").get_with_index(0).unwrap();

    let current_active_index = container_ids.iter().position(|&r| r == current_active.id()).unwrap();

    let new_active_index = if left_right {
        if current_active_index == 0 {
            container_ids.len() - 1
        } else {
            current_active_index - 1
        }
    } else {
        if current_active_index == container_ids.len() - 1 {
            0
        } else {
            current_active_index + 1
        }
    };

    //Redraw canvas if its selected, otherwise remove hover-info
    if new_active_index == 0 {
        draw_scene(planet, signal((false, 0.0, 0.0, 0.0, 0.0)).1, false, std::rc::Rc::new(std::cell::RefCell::new(None)));
    } else {
        set_mouse_properties((false, 0.0, 0.0, 0.0, 0.0));
    }

    current_active.class_list().remove_1("visible_element").unwrap();
    current_active.class_list().add_1("invisible_element").unwrap();
    if let Some(new_active) = document().get_element_by_id(container_ids[new_active_index]) {
        new_active.class_list().remove_1("invisible_element").unwrap();
        new_active.class_list().add_1("visible_element").unwrap();
    }
}

#[component]
pub fn SwitchView(planet: ReadSignal<PlanetData>, set_mouse_properties: WriteSignal<(bool, f64, f64, f64, f64)>) -> impl IntoView {
    view! {
        <button class="arrow" id="arrow_left" on:click=move |_| {switch_view(true, planet.get(), set_mouse_properties);}><Icon icon={i::AiCaretLeftFilled} /></button>
        <button class="arrow" id="arrow_right" on:click=move |_| {switch_view(false, planet.get(), set_mouse_properties);}><Icon icon={i::AiCaretRightFilled} /></button>
    }
}