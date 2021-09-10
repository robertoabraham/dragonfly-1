use compute::prelude::{
    arange, argmin, interp1d_linear_unchecked, linspace, ExtrapolationMode, Vector,
};
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

    let pnedatatilt = data.iter().map(|x| x.angle).collect::<Vector>();
    let pnedataflux = data.iter().map(|x| x.flux).collect::<Vector>();
    let pnedatafluxnorm = &pnedataflux / pnedataflux.max();

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

    let fractions = linspace(0., 0.5, 100);
    let shifts = linspace(-20., 20., 500);

    let mut results = fractions
        .par_iter()
        .map(|&frac| {
            let totalflux = &pnefluxout + frac * &pnefluxout_nii;
            let totalfluxnorm = &totalflux / totalflux.max();
            let shift_interp = |x: &[f64]| {
                interp1d_linear_unchecked(&pnetilt, &totalfluxnorm, x, ExtrapolationMode::Fill(0.))
            };
            let residual = |s: f64| {
                (shift_interp(&(&pnedatatilt - s)) - &pnedatafluxnorm)
                    .powi(2)
                    .sum()
            };
            let res_wrt_shifts = shifts.iter().map(|&x| residual(x)).collect::<Vector>();
            let min_idx = argmin(&res_wrt_shifts);

            (frac, shifts[min_idx], res_wrt_shifts[min_idx])
        })
        .collect::<Vec<_>>();

    let best = results
        .iter()
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap();

    println!("{:?}", best);
}
