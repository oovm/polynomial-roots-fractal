use indicatif::{ProgressBar, ProgressIterator};
use rayon::iter::ParallelBridge;
use crate::{copy_vec_ref, polynomial_eigenvalues};
use super::*;


pub struct LittlewoodTable {
    rank: usize,
    table: sled::Tree,
}

impl PolynomialRootsDatabase {
    pub fn littlewood_table(&self, rank: usize) -> std::io::Result<LittlewoodTable> {
        let tb: sled::Tree = self.database.open_tree(format!("Littlewood-{}", rank).as_bytes())?;
        Ok(LittlewoodTable { rank, table: tb })
    }
}


impl LittlewoodTable {
    pub fn update(&self, x: f32, y: f32) -> std::io::Result<()> {
        if x < 0.0 || y < 0.0 {
            return Ok(());
        }
        let x = (x * HALF_RESOLUTION) as u32;
        let y = (y * HALF_RESOLUTION) as u32;
        let point = Point { x, y };
        let mut counter = match self.table.get(&point)? {
            Some(s) => {
                // s.make_mut()?
                Counter { n: u32::from_le_bytes(s.as_ref().try_into().expect("size mismatch")) }
            }
            None => { Counter::default() }
        };
        counter.n += 1;
        self.table.insert(point, counter)?;
        Ok(())
    }
    pub fn evaluate(&self) -> std::io::Result<()> {
        println!("Calculating littlewood rank {}", self.rank);
        self.table.clear()?;
        let tasks = 2usize.pow(self.rank as u32);
        let bar = ProgressBar::new(tasks as u64);


        // let tp: Vec<Vec<f32>> = (0..2).map(|_| [-1.0f32, 1.0f32].into_iter()).multi_cartesian_product().collect_vec();
        let mut polynomials: Vec<Vec<f32>> = Vec::with_capacity(2usize.pow(self.rank as u32));
        for i in (0..self.rank).map(|_| [-1.0f32, 1.0f32].iter()).multi_cartesian_product().progress_count(polynomials.len() as u64) {
            polynomials.push(copy_vec_ref(i))
        }
        //println!("{}",polynomials.len());
        for i in polynomials.into_iter() {
            for e in polynomial_eigenvalues(i.as_slice()).iter() {
                self.update(e.re, e.im)?
            }
        }
        Ok(())
    }
}
