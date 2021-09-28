use std::process;

pub enum ImageType {
    Light,
    Dark,
}

// TODO: make this async!
pub fn expose(imagetype: ImageType, duration: f64, savepath: &str) {
    process::Command::new("dfcore").args(&[
        "expose",
        match imagetype {
            ImageType::Light => "",
            ImageType::Dark => "--dark",
        },
        "--duration",
        &duration.to_string(),
        "--file",
        savepath,
    ]);
}
