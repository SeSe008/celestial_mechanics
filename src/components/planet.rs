use leptos::prelude::*;
use leptos::ev::Event;
use wasm_bindgen::JsCast;

use crate::utils::presets::*;

#[derive(Clone, PartialEq)]
pub struct PlanetData {
    pub a_input: (ReadSignal<f64>, WriteSignal<f64>),
    pub a: (ReadSignal<f64>, WriteSignal<f64>),
    pub e: (ReadSignal<f64>, WriteSignal<f64>),
    pub m_sun: f64,
    pub m_object: (ReadSignal<f64>, WriteSignal<f64>),
    pub m_earth: f64,
    pub d: (ReadSignal<f64>, WriteSignal<f64>),
    pub d_earth: f64,
    pub g: f64
}

impl PlanetData {
    fn new(a_new: f64, e_new: f64, m_object_new: f64, d_new: f64) -> Self {
        let (a, set_a) = signal(a_new * 1.496 * (10.0_f64).powi(11));
        let (e, set_e) = signal(e_new);
        let (a_input, set_a_input) = signal(a_new);
        let (m_object, set_m_object) = signal(m_object_new);
        let (d, set_d) = signal(d_new);
        Self {
            a_input: (a_input, set_a_input),
            a: (a, set_a),
            e: (e, set_e),
            m_sun: 1.988416 * (10.0_f64).powi(30),
            m_object: (m_object, set_m_object),
            m_earth: 5.972168 * 10.0_f64.powi(24),
            d: (d, set_d),
            d_earth: 12742.46,
            g: 6.67430 * (10.0_f64).powi(-11)
        }
    }
}

pub fn create_planet(a: f64, e: f64, m: f64, d: f64) -> PlanetData {
    let planet = PlanetData::new(a, e, m, d);
    planet
} 

pub fn update_planet_data(planet: PlanetData, update_planet: WriteSignal<PlanetData>, ev: Event, index: i8) {
    let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
    if index == 0 {
        (planet.a_input).1.set(value);
        (planet.a).1.set(value * 1.496 * (10.0_f64).powi(11));
    } else if index == 1 {
        (planet.e).1.set(value);        
    } else if index == 2 {
        (planet.m_object).1.set(value);
    } else if index == 3 {
        (planet.d).1.set(value);
    }
    update_planet.set(planet);
}

#[component]
pub fn Inputs(planet_signal: (ReadSignal<PlanetData>, WriteSignal<PlanetData>)) -> impl IntoView {
    let presets = load_presets();
    view! {
        <div id="inputs">
            <div class="input_section">
                <div class="input_select input">
                    <span class="input_select_label">"Presets: "</span>
                    <select on:change=move |ev| {
                        let index = event_target_value(&ev).parse::<usize>().unwrap_or(0);
                        let input_sliders = document().get_elements_by_class_name("input_slider");

                        if index == 0 {
                            for i in 0..input_sliders.length() {
                                input_sliders.item(i).unwrap().class_list().remove_1("input_inactive").unwrap();
                            }
                            
                            planet_signal.1.set(create_planet(
                                document().get_element_by_id("input_slider_a").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number(),
                                document().get_element_by_id("input_slider_e").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number(),
                                document().get_element_by_id("input_slider_m").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number(),
                                document().get_element_by_id("input_slider_d").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value_as_number()
                            ));
                        } else {
                            for i in 0..input_sliders.length() {
                                input_sliders.item(i).unwrap().class_list().add_1("input_inactive").unwrap();
                            }

                            let preset = presets.get(index - 1).cloned().unwrap_or_else(|| PlanetPreset {
                                name: String::new(),
                                m: 0.0,
                                d: 0.0,
                                a: 0.0,
                                e: 0.0
                            });
                            planet_signal.1.set(create_planet(preset.a, preset.e, preset.m, preset.d));
                        }
                    }>
                        <option value="0">"Custom"</option>
                        {presets.iter().enumerate().map(|(index, preset)| {
                            view! {
                                <option value={(index + 1).to_string()}>{preset.name.clone()}</option>
                            }
                        }).collect_view()}
                    </select>
                </div>
            </div>
            <div class="input_section">
                <div class="input_slider input">
                    <span class="input_slider_label">"Semi-Major Axis"</span>
                    <input id="input_slider_a" type="range" min="1" max="750.0" step="0.01" value={planet_signal.0.get_untracked().a_input.0.get_untracked()} on:input=move |ev| {
                        update_planet_data(planet_signal.0.get(), planet_signal.1, ev, 0);
                    } />
                    <span class="input_slider_value">{move || format!("{:.2}", planet_signal.0.get().a_input.0.get())} "AU"</span>
                </div>
                <div class="input_slider input">
                    <span class="input_slider_label">"Orbit Eccentricity"</span>
                    <input id="input_slider_e" type="range" min="0" max=".99" step="0.01" value={planet_signal.0.get_untracked().e.0.get_untracked()} on:input=move |ev| {
                        update_planet_data(planet_signal.0.get(), planet_signal.1, ev, 1);  
                    } />
                    <span class="input_slider_value">{move || format!("{:.2}", planet_signal.0.get().e.0.get())}</span>
                </div>
            </div>
            <div class="input_section">
                <div class="input_slider input">
                    <span class="input_slider_label">"Object Mass"</span>
                    <input id="input_slider_m" type="range" min="0.01" max="330" step="0.01" value={planet_signal.0.get_untracked().m_object.0.get_untracked()} on:input=move |ev| {
                        update_planet_data(planet_signal.0.get(), planet_signal.1, ev, 2);
                    } />
                    <Show when=move || {planet_signal.0.get().m_object.0.get() != 0.0} fallback=|| {view!{<span class="input_slider_value">"Undefined"</span>}}>
                        <span class="input_slider_value">{move || format!("{:.2}", planet_signal.0.get().m_object.0.get())} "M🜨"</span>
                    </Show>
                </div>
                <div class="input_slider input">
                    <span class="input_slider_label">"Object Diameter"</span>
                    <input id="input_slider_d" type="range" min="0.01" max="12" step="0.01" value={planet_signal.0.get_untracked().d.0.get_untracked()} on:input=move |ev| {
                        update_planet_data(planet_signal.0.get(), planet_signal.1, ev, 3);
                    } />
                    <span class="input_slider_value">{move || format!("{:.2}", planet_signal.0.get().d.0.get())} "D🜨"</span>
                </div>
            </div>
        </div>
    }
}