use compute::prelude::{arange, Vector};
use csv::ReaderBuilder;
use dragonfly_rs::calibration::{get_tilt_shift, integrate_flux, Filter, Wavefront};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PNRecord {
    angle: f64,
    flux: f64,
}

fn main() {
    let rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("/home/js/programs/dragonfly-rs/data/PNeMeasurements/measure_source_301.txt")
        .expect("Could not open file as csv");

    let data = rdr
        .into_deserialize()
        .map(|x| x.expect("Could not deserialize field."))
        .collect::<Vec<PNRecord>>();

    let filter_cwl = 659.9;
    let pnetilt = arange(0., 20., 0.1);
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

    let total_flux = &pnefluxout + 0.12 * &pnefluxout_nii;

    for i in 0..pnetilt.len() {
        println!(
            "{},{},{},{}",
            pnetilt[i], pnefluxout[i], pnefluxout_nii[i], total_flux[i]
        );
    }
}
