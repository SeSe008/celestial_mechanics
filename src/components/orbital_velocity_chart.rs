use leptos::prelude::*;
use leptos_chartistry::*;
use leptos_chartistry::IntoInner;
use std::f64::consts::PI;

use crate::utils::orbital_velocity::calculate_orbital_velocity;
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

fn get_chart_data(a: f64, e: f64, m: f64, g: f64, start: f64, end: f64, step: f64) -> Vec<ChartData> {
    let mut data = Vec::new();
    let mut angle = start;
    while angle < end {
        let velocity = calculate_orbital_velocity(a, e, m, g, angle);
        if velocity <= 40.0 {
            data.push(ChartData::add(angle, velocity));
        }
        angle += step;
    }
    data
}

#[component]
pub fn VelocityChart(planet: ReadSignal<PlanetData>) -> impl IntoView {
    let chart_data = Memo::new(move |_| {
        get_chart_data(
            (planet.get().a).0.get(),
            (planet.get().e).0.get(),
            planet.get().m_sun,
            planet.get().g,
            0.0,
            2.0 * PI,
            0.01,
        )
    });

    let series = Series::new(|data: &ChartData| data.x)
        .line(Line::new(|data: &ChartData| data.y).with_name("Velocity (km/s):"))
        .with_y_range(0.0, 40.0)
        .with_x_range(0.0, 6.3);

    view! {
        <div id="velocity_chart">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=chart_data  
                top=RotatedLabel::middle("Orbital Velocity")
                left=vec![RotatedLabel::end("Velocity (km/s)").into(), TickLabels::aligned_floats().into()]
                bottom=vec![TickLabels::aligned_floats().into(), RotatedLabel::end("Radius (Rad)").into(),]
                inner=[
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    XGridLine::default().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
            />
        </div>
    }
}