use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrameData {
    pub angle: f64,
    pub raw_angle: f64,
    pub nobj: usize,
    pub spotflux: f64,
    pub spotarea: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(alias = "Area")]
    pub area: f64,
    #[serde(alias = "FluxAuto")]
    pub flux: f64,
    #[serde(alias = "Count")]
    pub count: usize,
}
