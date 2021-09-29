use compute::prelude::*;
use dragonfly_rs::core::*;
use dragonfly_rs::focuser::*;
use image2::Filter;
use std::io::Write;

fn main() {
    let mut data = fitsio::FitsFile::open("/tmp/out/test.fits").unwrap();
    let hdu = data.primary_hdu().unwrap();
    let shape = match &hdu.info {
        fitsio::hdu::HduInfo::ImageInfo { shape, .. } => shape,
        _ => panic!(),
    };
    let data: Vec<f64> = hdu.read_image(&mut data).unwrap();
    let data = Matrix::new(data, shape[1], shape[0]);

    // println!("{}", data);

    // let image = image2::Image::<u16, image2::Rgb>::open("/tmp/out/image2.jpg").unwrap();
    // let conv = image2::filter::convert();
    // let dest: image2::Image<u16, image2::Gray> = image.run(conv, None);

    // let k = image2::kernel::Kernel::laplacian();

    // let (w, h, c) = dest.shape();

    // let x = Matrix::new(
    //     dest.data.iter().map(|x| *x as f64).collect::<Vec<_>>(),
    //     h as i32,
    //     w as i32,
    // );

    // println!("{}", x);

    let lpl = conv2d(&data, LAPLACIAN_KERNEL_1);

    let mut writer = std::fs::File::create("/tmp/test.txt").unwrap();
    writeln!(&mut writer, "{}", lpl).unwrap();
    // expose::expose(expose::ImageType::Light, 0.1, "/code/out/test.fits");
}
