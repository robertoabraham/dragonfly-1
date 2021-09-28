use std::process;

pub enum ImageType {
    Light,
    Dark,
}

// TODO: make this async!
pub fn expose(imagetype: ImageType, duration: f64, savepath: &str) {
    let cmd = process::Command::new("dfcore")
        .args(&[
            "expose",
            "--duration",
            &duration.to_string(),
            "--file",
            savepath,
        ])
        .arg(match imagetype {
            ImageType::Light => "",

            ImageType::Dark => "--dark",
        })
        .spawn()
        .expect("Could not spawn dfcore::expose!");
}
