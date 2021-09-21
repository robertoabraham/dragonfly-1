use arrayvec::ArrayVec;
use lexical::parse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CatalogObject {
    //   outputs selected in dragonfly.param file
    //   1 NUMBER                 Running object number
    //   2 X_IMAGE                Object position along x                                    [pixel]
    //   3 Y_IMAGE                Object position along y                                    [pixel]
    //   4 XMIN_IMAGE             Minimum x-coordinate among detected pixels                 [pixel]
    //   5 XMAX_IMAGE             Maximum x-coordinate among detected pixels                 [pixel]
    //   6 YMIN_IMAGE             Minimum y-coordinate among detected pixels                 [pixel]
    //   7 YMAX_IMAGE             Maximum y-coordinate among detected pixels                 [pixel]
    //   8 FLUX_AUTO              Flux within a Kron-like elliptical aperture                [count]
    //   9 FLAGS                  Extraction flags
    //  10 FWHM_IMAGE             FWHM assuming a gaussian core                              [pixel]
    //  11 MAG_BEST               Best of MAG_AUTO and MAG_ISOCOR                            [mag]
    //  12 ISOAREA_IMAGE          Isophotal area above Analysis threshold                    [pixel**2]
    //  13 ELONGATION             A_IMAGE/B_IMAGE
    //  14 BACKGROUND             Background at centroid position                            [count]
    #[serde(skip_serializing, alias = "Number")]
    number: usize,
    #[serde(skip_serializing, alias = "XImage")]
    x_image: f64,
    #[serde(skip_serializing, alias = "YImage")]
    y_image: f64,
    #[serde(skip_serializing, alias = "XMinImage")]
    x_min_image: usize,
    #[serde(skip_serializing, alias = "YMinImage")]
    y_min_image: usize,
    #[serde(skip_serializing, alias = "XMaxImage")]
    x_max_image: usize,
    #[serde(skip_serializing, alias = "YMaxImage")]
    y_max_image: usize,
    #[serde(alias = "FluxAuto")]
    pub flux: f64,
    #[serde(skip_serializing, alias = "Flags")]
    flags: usize,
    #[serde(skip_serializing, alias = "FWHM")]
    fwhm: f64,
    #[serde(skip_serializing, alias = "MagBest")]
    mag_best: f64,
    #[serde(alias = "Area")]
    pub area: f64,
    #[serde(skip_serializing, alias = "AxialRatio")]
    axial_ratio: f64,
    #[serde(skip_serializing, alias = "Background")]
    background: f64,
}

pub fn run_sextractor(filepath: &str) -> Result<Vec<CatalogObject>, String> {
    match std::fs::metadata(filepath) {
        Ok(_) => {
            let proc = std::process::Command::new("sex")
                .args(&[
                    filepath,
                    "-c",
                    "/home/js/programs/dragonfly-rs/src/sextractor/dragonfly.sex",
                ])
                .output()
                .expect("Could not run SExtractor on file.");
            if proc.status.success() {
                let output_str = String::from_utf8_lossy(&proc.stdout);
                Ok(deserialize_sextractor(&output_str))
            } else {
                Err(String::from_utf8_lossy(&proc.stderr).to_string())
            }
        }
        Err(_) => panic!("File does not exist at {}.", filepath),
    }
}

pub fn deserialize_sextractor(output: &str) -> Vec<CatalogObject> {
    // takes in output from `run_sextractor`

    if output.is_empty() {
        return vec![];
    }

    output
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let av = line
                .trim()
                .split_whitespace()
                .collect::<ArrayVec<&str, 14>>();
            CatalogObject {
                number: parse::<usize, _>(av[0]).expect("Failed to parse number"),
                x_image: parse::<f64, _>(av[1]).expect("Failed to parse x_image."),
                y_image: parse::<f64, _>(av[2]).expect("Failed to parse y_image."),
                x_min_image: parse::<usize, _>(av[3]).expect("Failed to parse x_min_image."),
                y_min_image: parse::<usize, _>(av[4]).expect("Failed to parse y_min_image."),
                x_max_image: parse::<usize, _>(av[5]).expect("Failed to parse x_max_image."),
                y_max_image: parse::<usize, _>(av[6]).expect("Failed to parse y_max_image."),
                flux: parse::<f64, _>(av[7]).expect("Failed to parse flux."),
                flags: parse::<usize, _>(av[8]).expect("Failed to parse flags."),
                fwhm: parse::<f64, _>(av[9]).expect("Failed to parse fwhm."),
                mag_best: parse::<f64, _>(av[10]).expect("Failed to parse mag_best."),
                area: parse::<f64, _>(av[11]).expect("Failed to parse area."),
                axial_ratio: 1.
                    / parse::<f64, _>(av[12])
                        .expect("Failed to parse axial_ratio (1. / elongation)."),
                background: parse::<f64, _>(av[13]).expect("Failed to parse background."),
            }
        })
        .collect::<Vec<_>>()
}
