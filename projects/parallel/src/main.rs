use clap::Parser;
use nalgebra::{Complex, DMatrix};
use polynomial_roots::App;

fn main() -> std::io::Result<()> {
    App::parse().run().unwrap();
    Ok(())
}

pub fn polynomial_eigenvalues_complex(input: &[f32]) -> DMatrix<Complex<f32>> {
    let dim = input.len();
    DMatrix::from_fn(dim, dim, |r, c| {
        if r == 0 {
            Complex { re: -input[c], im: 0.0 }
        }
        else if r == c + 1 {
            Complex { re: 1.0, im: 0.0 }
        }
        else {
            Complex { re: 0.0, im: 0.0 }
        }
    })
}
