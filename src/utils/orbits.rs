use crate::components::orbit_visualization::RadiusPoint;
use web_sys::window;
use log::info;

pub fn add_orbit(points: &Vec<RadiusPoint>) {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(stored_data)) = storage.get_item("orbits") {
                match serde_json::from_str::<Vec<(i8, Vec<RadiusPoint>)>>(&stored_data) {
                    Ok(mut vec) => {
                        vec.push((vec.len() as i8, points.clone()));
                        storage.set_item("orbits", &serde_json::to_string(&vec).unwrap()).unwrap();
                    },
                    Err(e) => info!("Error deserializing data: {}", e),
                }
            }
            else {
                storage.set_item("orbits", &serde_json::to_string(&vec![(0, points.clone())]).unwrap()).unwrap();
            }
        }
    }
}  

pub fn load_orbits() -> Vec<(i8, Vec<RadiusPoint>)> {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(stored_data)) = storage.get_item("orbits") {
                match serde_json::from_str::<Vec<(i8, Vec<RadiusPoint>)>>(&stored_data) {
                    Ok(vec) => vec,
                    Err(e) => {
                        info!("{}", e);
                        vec![]
                    },
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

pub fn remove_last_orbit() {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(stored_data)) = storage.get_item("orbits") {
                match serde_json::from_str::<Vec<(i8, Vec<RadiusPoint>)>>(&stored_data) {
                    Ok(mut vec) => {
                        vec.remove(vec.len() - 1);
                        storage.set_item("orbits", &serde_json::to_string(&vec).unwrap()).unwrap();
                    },
                    Err(e) => info!("Error deserializing data: {}", e),
                }
            }
        }
    }
}

pub fn clear_orbits() {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            storage.set_item("orbits", &serde_json::to_string(&Vec::<Vec<(i8, Vec<RadiusPoint>)>>::new()).unwrap()).unwrap();
        }
    }
}