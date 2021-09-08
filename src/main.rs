use arrayvec::ArrayVec;
use clap::{Error, ErrorKind};
use compute::prelude::Vector;
use dragonfly_rs::{
    calibration::{Catalog, FrameData},
    utils::round_to_digits,
};

use std::{env, fs::remove_file, path::PathBuf, process::Command, time::Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Dragonfly Calibration",
    about = "Calibrates a filter-tilter unit."
)]
struct Opt {
    /// USB serial port number
    #[structopt(long, default_value = "COM5", name = "port_name")]
    port: String,
    /// Degrees of tilt to start the calibration at. Must be in [160., 200.] and greater than the
    /// end angle `end`.
    #[structopt(long, default_value = "160.", name = "start_angle")]
    start: f64,
    /// Degrees of tilt to end the calibration at. Must be in [160., 200.] and less than the
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
    #[structopt(short, long, requires = "tempdir")]
    keep: bool,
    /// Location to save the captured images.
    #[structopt(long)]
    tempdir: Option<PathBuf>,
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

    let start_time = Instant::now();

    let df_dir = env::var("DFREPOSITORIES");

    if df_dir.is_err() {
        Error::with_description(
            "Could not find the DFREPOSITORIES environment variable!",
            ErrorKind::EmptyValue,
        )
        .exit();
    }

    let df_dir = format!("{}\\Dragonfly-MaximDL\\", df_dir.unwrap());

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

            let tilt_result = Command::new("PowerShell")
                .args(&[
                    format!("{}PowerShell\\Send-FilterTilterCommand.ps1", df_dir).as_str(),
                    if opt.simulation {
                        "-simulation"
                    } else {
                        ""
                    },
                    "-Arg",
                    &format!("{}", current_angle),
                    "-Port",
                    &opt.port,
                ])
                .output()
                .expect("Could not run Send-FilterTilterCommand script!");

            let tilt_result = String::from_utf8_lossy(&tilt_result.stdout);

            if opt.verbose {
                println!("Tilt result: {}", tilt_result);
            }

            // this is super hacky. if we can rewrite send-filtertiltercommand in rust the error
            // handling would be MUCH nicer

            let tilt_result = tilt_result.split_whitespace().collect::<ArrayVec<_, 2>>();

            let raw_angle = lexical::parse(tilt_result[1].split(",OK").collect::<ArrayVec<_, 2>>()[0]).unwrap();

            let mut area = 0.;
            let mut flux = 0.;
            let mut nobj = 0;

            (0..opt.naverage).for_each(|j| {
                if opt.verbose {
                    println!("Taking image {} of {}", j + 1, opt.naverage);
                }

                println!("{}", df_dir);

                let result = Command::new("cscript")
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

                let result = String::from_utf8_lossy(&result.stdout);

                // TODO: arrayvec with fixed number of fields as extra error check
                let fields = result.split_whitespace().collect::<Vec<_>>();

                let filename = fields[1];

                if opt.verbose {
                    println!("Working on {}", filename);
                    println!("Analyzing the image to select the object with the largest area.");
                }

                let catalog = Command::new("PowerShell")
                    .args(&[format!("{}\\PowerShell\\New-ImageCatalog.ps1", df_dir).as_str(), filename])
                    .output();

                match catalog {
                    Ok(output) => {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        let mut parsed_output: Vec<Catalog> = serde_json::de::from_str(&output_str).expect("Could not parse output from New-ImageCatalog.ps1!");
                        parsed_output.sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap());
                        area += parsed_output[0].area;
                        flux += parsed_output[0].flux;
                        nobj += parsed_output[0].count;
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

            if opt.naverage > 1 {
                println!("Averaging results --- Angle: {:.2}\tAverageNObj: {:.0}\tAverageSpotFlux: {:.1}\tAverageArea{:.0}\tNAveraged: {:.0}", current_angle, nobj, flux, area, opt.naverage);
            }

            FrameData {
                angle: *current_angle,
                raw_angle,
                nobj,
                spotflux: round_to_digits(flux, 1),
                spotarea: round_to_digits(area, 1),
            }
        })
        .collect::<Vec<_>>();

    if opt.verbose {
        println!("{:?}", data);
    }

    let end_time = Instant::now();
    let elapsed = (end_time - start_time).as_secs();

    println!("{}", elapsed);
}
