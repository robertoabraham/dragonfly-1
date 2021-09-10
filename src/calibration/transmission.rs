use compute::prelude::{
    arange, interp1d_linear, trapezoid, Continuous, ExtrapolationMode, Normal, Vector,
};
use csv::Reader;
use lazy_static::lazy_static;
use rayon::prelude::*;
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

const TRANSMISSION_DATA_DIR: &str = "data/FilterTransmissionCurves";
const RFR_IDX_RATIO: f64 = 1. / 2.1;

pub fn load_transmission_data(filter: Filter) -> Vec<AOIRecord> {
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

lazy_static! {
    static ref TRANSMISSION_BPF08DEG0: Vec<AOIRecord> = load_transmission_data(Filter::Bpf08Deg0);
    static ref TRANSMISSION_BPF08DEG10: Vec<AOIRecord> = load_transmission_data(Filter::Bpf08Deg10);
    static ref TRANSMISSION_BPF31DEG0: Vec<AOIRecord> = load_transmission_data(Filter::Bpf31Deg0);
    static ref TRANSMISSION_BPF31DEG10: Vec<AOIRecord> = load_transmission_data(Filter::Bpf31Deg10);
}

pub fn get_transmission(filter: Filter, wavefront: Wavefront) -> (Vector, Vector) {
    let transmission: &'static [AOIRecord] = match filter {
        Filter::Bpf08Deg0 => &TRANSMISSION_BPF08DEG0,
        Filter::Bpf08Deg10 => &TRANSMISSION_BPF08DEG10,
        Filter::Bpf31Deg0 => &TRANSMISSION_BPF31DEG0,
        Filter::Bpf31Deg10 => &TRANSMISSION_BPF31DEG10,
    };

    transmission
        .iter()
        .map(|r| {
            (
                r.lambda_coll,
                match wavefront {
                    Wavefront::TCOLL => r.t_coll,
                    Wavefront::T3 => r.t_3deg,
                    Wavefront::T22 => r.t_22deg,
                },
            )
        })
        .unzip()
}

const FWHM: f64 = 2.3548200450309493;

pub fn get_tilt_shift(cwl: f64, tilts: &[f64]) -> Vector {
    cwl * (1.
        - ((RFR_IDX_RATIO) * (Vector::from(tilts) * std::f64::consts::PI / 180.).sin()).powi(2))
    .sqrt()
        - cwl
}

pub fn get_laser_spectrum(laser_cwl: f64, laser_fwhm: f64) -> (Vector, Vector) {
    let laser_std = laser_fwhm / FWHM;
    let laser_lambda = arange(650., 670., 0.01);
    let norm = Normal::new(laser_cwl, laser_std);
    let laser_flux = laser_lambda
        .iter()
        .map(|&x| norm.pdf(x))
        .collect::<Vector>();
    (laser_lambda, laser_flux)
}

pub fn integrate_flux(
    filter: Filter,
    filtershift: Option<f64>,
    wavefront: Wavefront,
    laser_cwl: f64,
    laser_fwhm: f64,
) -> f64 {
    let (d_wavelength, d_flux) = get_laser_spectrum(laser_cwl, laser_fwhm);
    let (mut t_wavelength, t_flux) = get_transmission(filter, wavefront);

    if let Some(shift) = filtershift {
        t_wavelength += shift;
    }

    let itflux = interp1d_linear(
        &t_wavelength,
        &t_flux,
        &d_wavelength,
        ExtrapolationMode::Fill(0.),
    );

    let (y, x): (Vector, Vector) = itflux
        .iter()
        .zip(d_flux)
        .zip(d_wavelength)
        .filter(|((&i_f, _), _)| i_f > 0.)
        .map(|((&i_f, d_f), d_w)| (i_f * d_f, d_w))
        .unzip();

    trapezoid(&y, Some(&x), None)
}

pub fn generate_model_transmission(stepsize: f64) -> (Vector, Vector, Vector) {
    let filter_cwl = 659.9;
    let pnetilt = arange(0., 20., stepsize);
    let pneshift = get_tilt_shift(filter_cwl, &pnetilt);

    let pnecwl = 656.3;
    let pnefluxout = pneshift
        .par_iter()
        .map(|&x| integrate_flux(Filter::Bpf31Deg0, Some(x), Wavefront::TCOLL, pnecwl, 0.61))
        .collect::<Vector>();

    let pnecwl_nii = 658.5;
    let pnefluxout_nii = pneshift
        .par_iter()
        .map(|&x| {
            integrate_flux(
                Filter::Bpf31Deg0,
                Some(x),
                Wavefront::TCOLL,
                pnecwl_nii,
                0.61,
            )
        })
        .collect::<Vector>();

    (pnetilt, pnefluxout, pnefluxout_nii)
}
