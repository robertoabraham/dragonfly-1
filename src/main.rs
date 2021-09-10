use arrayvec::ArrayVec;
use compute::prelude::{
    argmin, interp1d_linear_unchecked, linspace, Dot, ExtrapolationMode, Vector,
};
use csv::ReaderBuilder;
use dragonfly_rs::calibration::{
    generate_model_transmission, MODEL_FLUX_COARSE, MODEL_FLUX_NII_COARSE, MODEL_TILT_COARSE,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PNRecord {
    angle: f64,
    flux: f64,
}

fn main() {
    let bests = (1..4)
        .map(|i| {
            let rdr = ReaderBuilder::new()
                .delimiter(b' ')
                .has_headers(false)
                .from_path(
                    format!(
                    "./data/PNeMeasurements/measure_source_30{}.txt",
                    i
                )
                    .as_str(),
                )
                .expect("Could not open file as csv");

            let data = rdr
                .into_deserialize()
                .map(|x| x.expect("Could not deserialize field."))
                .collect::<Vec<PNRecord>>();

            let pnedatatilt = data.iter().map(|x| x.angle).collect::<Vector>();
            let pnedataflux = data.iter().map(|x| x.flux).collect::<Vector>();
            let pnedatafluxnorm = &pnedataflux / pnedataflux.max();

            let fractions = linspace(0., 0.5, 100);
            let shifts = linspace(-10., 10., 500);

            let mut result = fractions
                .par_iter()
                .map(|&frac| {
                    let totalflux = MODEL_FLUX_COARSE
                        .iter()
                        .zip(MODEL_FLUX_NII_COARSE)
                        .map(|(x, y)| x + frac * y)
                        .collect::<Vector>();
                    let totalfluxnorm = &totalflux / totalflux.max();
                    let shift_interp = |x: &[f64]| {
                        interp1d_linear_unchecked(
                            &MODEL_TILT_COARSE,
                            &totalfluxnorm,
                            x,
                            ExtrapolationMode::Fill(0.),
                        )
                    };
                    let residual = |s: f64| {
                        (shift_interp(&(&pnedatatilt - s)) - &pnedatafluxnorm)
                            .powi(2)
                            .sum()
                    };
                    let res_wrt_shifts =
                        shifts.par_iter().map(|&x| residual(x)).collect::<Vector>();
                    let min_idx = argmin(&res_wrt_shifts);

                    (frac, shifts[min_idx], res_wrt_shifts[min_idx])
                })
                .collect::<Vec<_>>();

            let best = result
                .iter()
                .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
                .unwrap()
                .to_owned();

            best
        })
        .collect::<ArrayVec<_, 3>>();

    let strengths = Vector::from([bests[0].0, bests[1].0, bests[2].0]);
    let shifts = Vector::from([bests[0].1, bests[1].1, bests[2].1]);
    let weights = 1. / Vector::from([bests[0].2, bests[1].2, bests[2].2]).powi(2);

    println!(
        "Nitrogen relative strength: {}",
        weights.dot(strengths) / weights.sum(),
    );
    println!("Best angle shift: {}", weights.dot(shifts) / weights.sum());
}
