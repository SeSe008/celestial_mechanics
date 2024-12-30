use leptos::prelude::*;
use leptos_chartistry::*;
use leptos_chartistry::IntoInner;
use std::f64::consts::PI;

use crate::utils::gravitational_force_sun::calculate_gravitational_force_with_sun;
use super::planet::PlanetData;

#[derive(Clone, PartialEq)]
struct ChartData {
    x: f64,
    y: f64
}

impl ChartData {
    fn add(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn get_chart_data(m_object_earth_ratio: f64, m_sun: f64, m_earth: f64, a: f64, e: f64, g: f64, start: f64, end: f64, step: f64) -> Vec<ChartData> {
    let mut data = Vec::new();
    let mut angle = start;
    while angle < end {
        let force = calculate_gravitational_force_with_sun(m_object_earth_ratio, m_sun, m_earth, a, e, g, angle);
        if force < 4.6 * 10.0_f64.powi(23) {
            data.push(ChartData::add(angle, force));
        }
        angle += step;
    }
    data
}

#[component]
pub fn GravitationalForceWithSunChart(planet: ReadSignal<PlanetData>) -> impl IntoView {
    let chart_data = Memo::new(move |_| {
        get_chart_data(
            planet.get().m_object.0.get(),
            planet.get().m_sun,
            planet.get().m_earth,
            planet.get().a.0.get(),
            planet.get().e.0.get(),
            planet.get().g,
            0.0,
            2.0 * PI,
            0.01,
        )
    });

    let series = Series::new(|data: &ChartData| data.x)
        .line(Line::new(|data: &ChartData| data.y).with_name("Force (N):"))
        .with_y_range(0.0, 4.6 * 10.0_f64.powi(23))
        .with_x_range(0.0, 6.3);

    view! {
        <div id="gravitational_force_with_sun_chart" class="invisible_element">
            <Show when=move || planet.get().m_object.0.get() != 0.0
                fallback=|| view!{<div class="small_property">"Mass is undefined"</div>}>
                <Chart
                    aspect_ratio=AspectRatio::from_env()
                    series=series.clone()
                    data=chart_data  
                    top=RotatedLabel::middle("Gravitational Force with Sun")
                    left=vec![RotatedLabel::end("Force (N)").into(), TickLabels::default().with_format(|value: &f64, _| format!("{:.2e}", value)).into()]
                    bottom=vec![TickLabels::aligned_floats().into(), RotatedLabel::end("Radius (Rad)").into()]
                    inner=vec![
                        AxisMarker::left_edge().into_inner(),
                        AxisMarker::bottom_edge().into_inner(),
                        XGridLine::default().into_inner(),
                        YGridLine::default().into_inner(),
                        YGuideLine::over_mouse().into_inner(),
                        XGuideLine::over_data().into_inner(),
                    ]
                />
            </Show>
        </div>
    }
}