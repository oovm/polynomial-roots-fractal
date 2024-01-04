use clap::Parser;
use itertools::Itertools;
use nalgebra::{Complex, DMatrix};
use polynomial_roots::{copy_vec_ref, polynomial_eigenvalues, App, PolynomialRootsDatabase};
use std::{fs::File, io::Write, path::Path};
use wolfram_wxf::{ToWolfram, WolframValue};
// use rayon::prelude::*;

fn main() {
    App::parse().run().unwrap();

    // let root = PolynomialRootsDatabase::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    //
    // for rank in 2..=20 {
    //     root.littlewood_table(rank)?.evaluate()?
    // }
    // Ok(())
}

#[test]
fn test() {
    polynomial_roots_export(5).unwrap_or_default();
}

fn polynomial_roots_export(rank: usize) -> std::io::Result<()> {
    let roots = polynomial_roots(rank);
    let mut file = File::create(format!("polynomial_roots_{}.wxf", rank))?;
    file.write_all(&WolframValue::list(roots).to_compressed())?;
    Ok(())
}

pub fn polynomial_roots(rank: usize) -> Vec<WolframValue> {
    assert!(rank > 1);
    // let tp: Vec<Vec<f32>> = (0..2).map(|_| [-1.0f32, 1.0f32].into_iter()).multi_cartesian_product().collect_vec();
    let mut polynomials: Vec<Vec<f32>> = Vec::with_capacity(2usize.pow(rank as u32));
    let mut roots = Vec::with_capacity(rank * 2usize.pow(rank as u32));
    for i in (0..rank).map(|_| [-1.0f32, 1.0f32].iter()).multi_cartesian_product() {
        polynomials.push(copy_vec_ref(i))
    }
    // println!("{}",polynomials.len());
    for i in polynomials.into_iter() {
        for e in polynomial_eigenvalues(i.as_slice()).iter() {
            roots.push(WolframValue::list(vec![e.im.to_wolfram(), e.re.to_wolfram()]))
        }
    }
    // println!("{}",roots.len());
    return roots;
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
