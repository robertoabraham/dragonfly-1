use arrayvec::ArrayVec;
use clap::{Error, ErrorKind};
use compute::prelude::{argmin, interp1d_linear_unchecked, linspace, ExtrapolationMode, Vector};
use dragonfly::{calibration::{
        FTAction, FTCommand, FrameData, MODEL_FLUX, MODEL_FLUX_NII, MODEL_TILT,
    }, sextractor::{CatalogObject, run_sextractor}, utils::round_to_digits};
use rayon::prelude::*;

use std::{env, fs::remove_file, process::Command, thread::current};
use structopt::{
    clap::AppSettings::{ColorAuto, ColoredHelp},
    StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Dragonfly: Calibration",
    about = "Calibrates a filter-tilter unit.",
    author,
)]
#[structopt(setting(ColorAuto), setting(ColoredHelp))]
struct Opt {
    /// USB serial port number
    #[structopt(long, default_value = "COM5", name = "port_name")]
    port: String,
    /// Degrees of tilt to start the calibration at. Must be in [160., 200.] and less than the
    /// end angle `end`.
    #[structopt(long, default_value = "160.", name = "start_angle")]
    start: f64,
    /// Degrees of tilt to end the calibration at. Must be in [160., 200.] and greater than the
    /// start angle `start`.
    #[structopt(long, default_value = "200.", name = "end_angle")]
    end: f64,
    /// Number of steps to take between the start and end angle (i.e., larger value means a higher
    /// resolution). Must be at least 2.
    #[structopt(long, default_value = "30")]
    nstep: usize,
    /// Time in seconds for each exposure.
    #[structopt(long, default_value = "60.", name = "exposure_seconds")]
    exptime: f64,
    /// Whether to run in simulation mode (expose, but don't tilt).
    #[structopt(short, long)]
    simulation: bool,
    /// Number of frames to average over at each tilt angle.
    #[structopt(long, default_value = "1")]
    naverage: usize,
    /// Whether to save the captured images.
    #[structopt(short, long)] // , requires = "tempdir")]
    keep: bool,
    // /// Location to save the captured images.
    // #[structopt(long)]
    // tempdir: Option<PathBuf>,
    /// Whether to be verbose and print messages.
    #[structopt(long, short = "v")]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.start < 160. || opt.start > 200. {
        Error::with_description(
            "Start angle must be at least 160 and less than 200 degrees.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if opt.end < 160. || opt.end > 200. {
        Error::with_description(
            "End angle must be at least 160 and less than 200 degrees.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if opt.start > opt.end {
        Error::with_description(
            "Start angle must be less than end angle.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if opt.nstep < 2 {
        Error::with_description(
            "Number of steps must be at least 2.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if opt.exptime <= 0. {
        Error::with_description("Exposure time must be positive.", ErrorKind::InvalidValue).exit()
    }

    let df_dir = "/tmp";

    // let df_dir = env::var("DFREPOSITORIES");
    // if df_dir.is_err() {
    //     Error::with_description(
    //         "Could not find the DFREPOSITORIES environment variable!",
    //         ErrorKind::EmptyValue,
    //     )
    //     .exit();
    // }
    // let df_dir = df_dir.unwrap();

    // let df_dir = format!("{}\\Dragonfly-MaximDL\\", df_dir);

    let stepsize = (opt.end - opt.start) / (opt.nstep - 1) as f64;
    let raw_angles = (0..opt.nstep)
        .map(|x| round_to_digits(opt.start + x as f64 * stepsize, 1))
        .collect::<Vector>();

    if opt.verbose {
        println!("Raw angles: {:?}", raw_angles);
    }

    let data = raw_angles
        .iter()
        .enumerate()
        .map(|(i, current_angle)| {
            
            if opt.verbose {
                println!("Iteration {} of {}", i + 1, opt.nstep);
            }
            
            let tilt_command = FTAction {
                command: FTCommand::GET,
                value: *current_angle,
                portname: &opt.port,
                simulation: Some(format!("{}/whatever-{}.txt", df_dir, alea::u32())),
                verbose: true,
            };

            let tilt_result = tilt_command.run().expect("Filter tilter command failed!");

            if opt.verbose {
                println!("Tilt result: {:?} {}", tilt_result.0, tilt_result.1);
            }

            let raw_angle = tilt_result.1;

            let mut area = 0.;
            let mut flux = 0.;
            let mut nobj = 0;

            (0..opt.naverage).for_each(|j| {
                if opt.verbose {
                    println!("Taking image {} of {}", j + 1, opt.naverage);
                }

                let result = if opt.simulation {
                    format!("Saved /home/js/programs/dragonfly/data/LaserCalibration/DRAGONFLY301_{}_light.fits", i + 1)
                } else {
                    let expose = Command::new("cscript")
                        .args(&[
                            "/nologo",
                            &format!("{}\\VBScript\\Expose.vbs", df_dir),
                            "light",
                            &format!("{}", opt.exptime),
                            &format!("/tiltgoal:{}", current_angle),
                            &format!("/rawtilt:{}", raw_angle),
                        ])
                        .output()
                        .expect("Could not run expose.vbs script!");

                    String::from_utf8_lossy(&expose.stdout).to_string()
                };

                let fields = result.split_whitespace().collect::<ArrayVec<_, 2>>();

                let filename = fields[1];

                if opt.verbose {
                    println!("Working on {}", filename);
                    println!("Analyzing the image to select the object with the largest area.");
                }

                let catalog = run_sextractor(filename);

                match catalog {
                    Ok(mut output) => {
                        if !output.is_empty() {
                            output.sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap());
                            area += output[0].area;
                            flux += output[0].flux;
                            nobj += output.len();
                        } else {
                            if opt.verbose {
                                println!("No sources detected.");
                            }
                        }
                    },
                    Err(_) => {
                        if opt.verbose {
                            println!("No sources detected.");
                        }
                    }
                }

                if opt.verbose {
                    println!(
                        "Iteration: {}\tAngle: {}\tNObj: {}\tSpotFlux: {}\tArea: {}",
                        j + 1,
                        current_angle,
                        nobj,
                        flux,
                        area
                    );
                }

                if !opt.keep {
                    if opt.verbose {
                        println!("Deleting image at {}", filename);
                    }
                    remove_file(filename).expect("Could not remove file!");
                }
            });

            area /= opt.naverage as f64;
            flux /= opt.naverage as f64;
            nobj /= opt.naverage;

            if opt.verbose && opt.naverage > 1 {
                println!("Averaging results --- Angle: {:.2}\tAverageNObj: {:.0}\tAverageSpotFlux: {:.1}\tAverageArea{:.0}\tNAveraged: {:.0}", current_angle, nobj, flux, area, opt.naverage);
            }

            FrameData {
                angle: *current_angle,
                raw_angle,
                nobj,
                spotflux: flux,
                spotarea: area,
            }
        })
        .collect::<Vec<_>>();

    if opt.verbose {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }

    let datatilt = data.iter().map(|x| x.angle - 180.).collect::<Vector>();
    let dataflux = data.iter().map(|x| x.spotflux).collect::<Vector>();
    let datafluxnorm = &dataflux / dataflux.max();

    println!("{:?}", datatilt);
    println!("{:?}", datafluxnorm);

    let fractions = linspace(0., 1., 100);
    let shifts = linspace(-25., 25., 500);

    let result = fractions
        .par_iter()
        .map(|&frac| {
            let totalflux = {
                MODEL_FLUX
                    .iter()
                    .zip(MODEL_FLUX_NII)
                    .map(|(x, y)| x + frac * y)
                    .collect::<Vector>()
            };
            let totalfluxnorm = &totalflux / totalflux.max();
            let shift_interp = |x: &[f64]| {
                interp1d_linear_unchecked(
                    &MODEL_TILT,
                    &totalfluxnorm,
                    x,
                    ExtrapolationMode::Fill(0., 0.),
                )
            };
            let residual = |s: f64| {
                ((shift_interp(&(&datatilt - s)) - &datafluxnorm)
                    .powi(2)
                    * (1. + &datafluxnorm))
                    .sum()
            };
            let res_wrt_shifts = shifts.par_iter().map(|&x| residual(x)).collect::<Vector>();
            let min_idx = argmin(&res_wrt_shifts);

            (frac, shifts[min_idx], res_wrt_shifts[min_idx])
        })
        .collect::<Vec<_>>();

    let best = result
        .iter()
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
        .to_owned();

    println!("Nii strength: {}", best.0);
    println!("Tilt shift: {}", best.1);
}
