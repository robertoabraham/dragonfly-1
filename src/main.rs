use dragonfly_rs::sextractor::*;

fn main() {
    let sexres = run_sextractor(
        "/home/js/programs/dragonfly-rs/data/LaserCalibration/DRAGONFLY301_30_light.fits",
    )
    .unwrap();
    println!("{:#?}", sexres);
}
