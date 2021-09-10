use std::fs::File;

use csv::{Reader, ReaderBuilder};
use dragonfly_rs::calibration::{load_iridian_data, Filter, FrameData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PNRecord {
    angle: f64,
    flux: f64,
}

fn main() {
    // let file = File::open("/home/js/programs/dragonfly-rs/sim.txt")
    //     .expect("Could not read file containing calibration data.");
    // let calibration_data: Vec<FrameData> =
    //     serde_json::de::from_reader(file).expect("Could not serialize calibration data.");

    // for ele in calibration_data {
    //     println!("{},{}", ele.raw_angle, ele.spotflux);
    // }

    let rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("/home/js/programs/dragonfly-rs/data/PNeMeasurements/measure_source_301.txt")
        .expect("Could not open file as csv");

    let data = rdr
        .into_deserialize()
        .map(|x| x.expect("Could not deserialize field."))
        .collect::<Vec<PNRecord>>();

    for i in data {
        println!("{:?}", i);
    }

    // let iridian = load_iridian_data(Filter::Bpf08Deg0);

    // for i in iridian {
    //     println!("{:?}", i);
    // }
}
