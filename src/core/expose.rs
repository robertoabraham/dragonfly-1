use std::process;

pub enum ImageType {
    Light,
    Dark,
}

// TODO: make this async!
pub fn expose(imagetype: ImageType, duration: f64, savepath: &str) {
    let cmd = match imagetype {
        ImageType::Light => process::Command::new("dfcore")
            .args(&[
                "expose",
                "--duration",
                &duration.to_string(),
                "--file",
                savepath,
            ])
            .spawn(),
        ImageType::Dark => process::Command::new("dfcore")
            .args(&[
                "expose",
                "--dark",
                "--duration",
                &duration.to_string(),
                "--file",
                savepath,
            ])
            .spawn(),
    }
    .expect("Could not spawn dfcore::expose!");
}
