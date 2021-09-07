use clap::{Error, ErrorKind};
use std::path::PathBuf;
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
}

fn main() {
    let opt = Opt::from_args();
    if (opt.start < 160. || opt.start > 200.) {
        Error::with_description(
            "Start angle must be at least 160 and less than 200 degrees.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if (opt.end < 160. || opt.end > 200.) {
        Error::with_description(
            "End angle must be at least 160 and less than 200 degrees.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
    if (opt.start > opt.end) {
        Error::with_description(
            "Start angle must be less than end angle.",
            ErrorKind::InvalidValue,
        )
        .exit()
    }
}
