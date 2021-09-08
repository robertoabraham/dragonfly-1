use std::{
    fs::File,
    io::{Read, Write},
    time::Duration,
};

use chrono::Utc;
use clap::Error;
use serde::{Deserialize, Serialize};
use serialport::SerialPort;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrameData {
    pub angle: f64,
    pub raw_angle: f64,
    pub nobj: usize,
    pub spotflux: f64,
    pub spotarea: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CatalogObject {
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
    ymaximage: usize,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FTCommand {
    GET,
    SET,
    GETRAW,
    SETRAW,
    ZERO,
    GETZERO,
    SETZERO,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FTCommandResult {
    A,
    R,
    Z,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FTAction<'a> {
    pub command: FTCommand,
    pub value: f64,
    pub portname: &'a str,
    pub simulation: Option<String>,
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedFTData {
    pub description: String,
    pub zeropoint: f64,
    pub rawangle: f64,
    pub date: chrono::DateTime<Utc>,
}

impl<'a> FTAction<'a> {
    pub fn run(&self) -> Result<(FTCommandResult, f64), Error> {
        if let Some(path) = &self.simulation {
            let file = File::open(path);
            let mut sim = match file {
                Ok(f) => {
                    if self.verbose {
                        println!("Serializing simulated data from {}", path);
                    }
                    let res: SimulatedFTData = serde_json::de::from_reader(f)
                        .expect("Could not deserialize simulated FT data!");
                    if self.verbose {
                        println!("Simulated zero point: {}", res.zeropoint);
                        println!("Simulated raw angle: {}", res.rawangle);
                    }
                    res
                }
                Err(_) => match File::create(path) {
                    Ok(f) => {
                        if self.verbose {
                            println!("Creating new simulated data file with zero point 0.0 and raw angle 171.0.");
                        }
                        let sim = SimulatedFTData {
                            description: "Simulated position file.".to_owned(),
                            zeropoint: 0.,
                            rawangle: 171.,
                            date: chrono::Utc::now(),
                        };
                        let res = serde_json::ser::to_writer(f, &sim);
                        if res.is_err() {
                            println!("Could not serialize simulated FT data to file!");
                        }
                        sim
                    }
                    Err(_) => panic!("Could not read/write file for simulation!"),
                },
            };

            let output = match self.command {
                FTCommand::GET => {
                    sim.rawangle = self.value + sim.zeropoint;
                    (FTCommandResult::A, sim.rawangle - sim.zeropoint)
                }
                FTCommand::SET => {
                    sim.rawangle = self.value + sim.zeropoint;
                    (FTCommandResult::A, self.value)
                }
                FTCommand::GETRAW => (FTCommandResult::R, sim.rawangle),
                FTCommand::SETRAW => {
                    sim.rawangle = self.value;
                    (FTCommandResult::R, self.value)
                }
                FTCommand::ZERO => {
                    sim.zeropoint = sim.rawangle;
                    (FTCommandResult::Z, sim.zeropoint)
                }
                FTCommand::GETZERO => (FTCommandResult::Z, sim.zeropoint),
                FTCommand::SETZERO => {
                    sim.zeropoint = self.value;
                    (FTCommandResult::Z, sim.zeropoint)
                }
            };

            // if self.verbose {
            //     println!("Simulation: {:?}", sim);
            // }

            if let Ok(f) = File::create(path) {
                if self.verbose {
                    println!("Serializing output to {}", path);
                }
                serde_json::ser::to_writer(f, &sim).expect("Could not serialize output");
            }

            Ok(output)
        } else {
            let mut port = serialport::new(self.portname, 9600)
                .parity(serialport::Parity::None)
                .data_bits(serialport::DataBits::Eight)
                .stop_bits(serialport::StopBits::One)
                .timeout(Duration::from_millis(5000))
                .open_native()
                .expect(format!("Could not open serial port {}", self.portname).as_str());

            port.clear(serialport::ClearBuffer::All)
                .expect("Could not clear serial port input/output buffers!");
            port.write_fmt(format_args!("{} {}", stringify!(self.command), self.value))
                .expect("Could not write to serial port!");

            let mut output = "".to_owned();
            port.read_to_string(&mut output)
                .expect("Could not read from serial port!");

            todo!()
        }
    }
}
