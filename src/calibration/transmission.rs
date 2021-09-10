use compute::prelude::Vector;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AOIRecord {
    #[serde(alias = "lambdacoll")]
    pub lambda_coll: f64,
    #[serde(alias = "lambda22deg")]
    pub lambda_22deg: f64,
    #[serde(alias = "lambda3deg")]
    pub lambda_3deg: f64,
    #[serde(alias = "Tcoll")]
    pub t_coll: f64,
    #[serde(alias = "T22deg")]
    pub t_22deg: f64,
    #[serde(alias = "T3deg")]
    pub t_3deg: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Wavefront {
    TCOLL,
    T3,
    T22,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Filter {
    Bpf08Deg0,
    Bpf08Deg10,
    Bpf31Deg0,
    Bpf31Deg10,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterTilter {
    // central wavelength
    pub cwl: f64,
    pub tilts: Vector,
}

const TRANSMISSION_DATA_DIR: &str = "data/FilterTransmissionCurves";
const RFR_IDX_RATIO: f64 = 1. / 2.1;

pub fn load_iridian_data(filter: Filter) -> Vec<AOIRecord> {
    let fp = match filter {
        Filter::Bpf08Deg0 => format!("{}/0.8BPF_0deg.csv", TRANSMISSION_DATA_DIR),
        Filter::Bpf08Deg10 => format!("{}/0.8BPF_10deg.csv", TRANSMISSION_DATA_DIR),
        Filter::Bpf31Deg0 => format!("{}/3.1BPF_0deg.csv", TRANSMISSION_DATA_DIR),
        Filter::Bpf31Deg10 => format!("{}/3.1BPF_10deg.csv", TRANSMISSION_DATA_DIR),
    };

    let rdr = Reader::from_path(fp).expect("Could not open file as csv");
    rdr.into_deserialize()
        .map(|x| x.expect("Could not deserialize field."))
        .collect::<Vec<AOIRecord>>()
}

impl FilterTilter {
    /// cwl: central wavelength
    /// theta: angle of incidence in degrees
    pub fn shift(cwl: f64, theta: f64) -> f64 {
        cwl * (1. - ((RFR_IDX_RATIO) * (theta * std::f64::consts::PI / 180.).sin()).powi(2)).sqrt()
            - cwl
    }

    pub fn get_tilt_shift(&self) -> Vector {
        self.cwl
            * (1. - ((RFR_IDX_RATIO) * (&self.tilts * std::f64::consts::PI / 180.).sin()).powi(2))
                .sqrt()
            - self.cwl
    }
}
