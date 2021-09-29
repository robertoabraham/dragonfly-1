use compute::prelude::{dot, Matrix};
use rayon::prelude::*;

pub const LAPLACIAN_KERNEL_1: [f64; 9] = [0., 1., 0., 1., -4., 1., 0., 1., 0.];
pub const LAPLACIAN_KERNEL_2: [f64; 9] = [1., 1., 1., 1., 8., 1., 1., 1., 1.];

/// Performs a naive 2d convolution on a signal given some kernel.
/// Assumes that the kernel has an odd side length (e.g., 3x3).
pub fn conv2d<const N: usize>(signal: &Matrix, kernel: [f64; N]) -> Matrix {
    let [h, w] = signal.shape();

    let ksize = (kernel.len() as f32).sqrt();

    assert!(ksize % 1. == 0., "Kernel is not square.");

    let ksize = ksize as usize;
    let ncrop = ksize / 2;

    // slide down image

    // println!("h = {}, w = {}", h, w);

    let conv = (0..h - 2 * ncrop)
        .into_par_iter()
        .map(|i| {
            // slide across horizontally
            (0..w - 2 * ncrop).map(move |j| {
                // get block to convolve with. cache: yikes
                // go down each row in the block (and the kernel)
                // println!("block i = {}, j = {}", i, j);
                (0..ksize)
                    // .into_par_iter()
                    .map(|r| {
                        // println!("kernel chunk, row {}, col {} to {}", i + r, j, j + ksize);
                        dot(
                            &kernel[(r * ksize)..(r + 1) * ksize],
                            &signal[i + r][j..j + ksize],
                        )
                    })
                    .sum()
            })
            // .collect::<Vec<_>>()
        })
        // .flatten()
        .flatten_iter()
        .collect::<Vec<_>>();

    let output = Matrix::new(conv, w - 2 * ncrop, h - 2 * ncrop);

    return output;
}
