#[derive(Clone)]
pub struct PlanetPreset {
    pub name: String,
    pub m: f64,
    pub d: f64,
    pub a: f64,
    pub e: f64,
}


pub fn load_presets() -> Vec<PlanetPreset> {
    return vec![
        PlanetPreset {
            name: String::from("Mercury"),
            m: 0.0553,
            d: 0.383,
            a: 0.387,
            e: 0.206
        },
        PlanetPreset {
            name: String::from("Venus"),
            m: 0.0553,
            d: 0.949,
            a: 0.72,
            e: 0.007
        },
        PlanetPreset {
            name: String::from("Earth"),
            m: 1.0,
            d: 1.0,
            a: 1.00,
            e: 0.017
        },
        PlanetPreset {
            name: String::from("Mars"),
            m: 0.107,
            d: 0.532,
            a: 1.51,
            e: 0.094
        },
        PlanetPreset {
            name: String::from("Jupiter"),
            m: 317.8,
            d: 11.21,
            a: 5.19,
            e: 0.049
        },
        PlanetPreset {
            name: String::from("Saturn"),
            m: 95.2,
            d: 9.45,
            a: 9.54,
            e: 0.052
        },
        PlanetPreset {
            name: String::from("Uranus"),
            m: 14.5,
            d: 4.01,
            a: 19.18,
            e: 0.047
        },
        PlanetPreset {
            name: String::from("Neptune"),
            m: 17.1,
            d: 3.88,
            a: 30.08,
            e: 0.01
        },
        PlanetPreset {
            name: String::from("Pluto"),
            m: 0.0022,
            d: 0.187,
            a: 39.08,
            e: 0.244
        },
        PlanetPreset {
            name: String::from("Haumea"),
            m: 0.00066,
            d: 0.125,
            a: 43.10,
            e: 0.2
        },
        PlanetPreset {
            name: String::from("Quaoar"),
            m: 0.0002,
            d: 0.0852,
            a: 43.33,
            e: 0.04
        },
        PlanetPreset {
            name: String::from("Makemake"),
            m: 0.000519,
            d: 0.112,
            a: 45.50,
            e: 0.16
        },
        PlanetPreset {
            name: String::from("Gonggong"),
            m: 0.000293,
            d: 0.0965,
            a: 67.07,
            e: 0.5
        },
        PlanetPreset {
            name: String::from("Eris"),
            m: 0.0027,
            d: 0.183,
            a: 67.84,
            e: 0.43
        },
        PlanetPreset {
            name: String::from("Sedna"),
            m: 0.0,
            d: 0.0781,
            a: 506.0,
            e: 0.85
        },
        PlanetPreset {
            name: String::from("Leleākūhonua"),
            m: 0.0,
            d: 0.0173,
            a: 1089.65,
            e: 0.952
        },
        PlanetPreset {
            name: String::from("2014 FE72"),
            m: 0.0,
            d: 0.0212,
            a: 2044.34,
            e: 0.983
        },
    ];
}