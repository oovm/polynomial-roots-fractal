use super::*;
use aberth::AberthSolver;
use std::ops::Add;

pub struct LittlewoodTable {
    rank: u32,
    table: sled::Tree,
}

impl PolynomialRootsDatabase {
    pub fn littlewood_table(&self, rank: u32) -> std::io::Result<LittlewoodTable> {
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
            None => Counter::default(),
        };
        counter.n += 1;
        self.table.insert(point, counter)?;
        Ok(())
    }
    pub fn evaluate(&self) -> std::io::Result<()> {
        self.table.clear()?;
        let tasks = 2u32.pow(self.rank);
        println!("Calculating littlewood rank {} with {} tasks", self.rank, tasks);

        let bar = {
            let bar = ProgressBar::new(tasks as u64);
            bar.set_style(
                ProgressStyle::with_template("{bar:100.cyan/blue} [Time {elapsed_precise}, ETA {eta_precise}]").unwrap(),
            );
            bar
        };
        (0..tasks).into_par_iter().for_each(|i| {
            let mut solver = AberthSolver::new();
            solver.epsilon = 0.01 / HALF_RESOLUTION;
            solver.max_iterations = 16;
            let equation = make_equation(i as u64, self.rank as u64);
            // let roots = polynomial_eigenvalues(&equation);
            for root in solver.find_roots(&equation).iter() {
                self.update(root.re, root.im).unwrap();
            }
            bar.inc(1);
        });
        bar.finish();
        Ok(())
    }
}

fn make_equation(index: u64, order: u64) -> Vec<f32> {
    let mut result = Vec::with_capacity(order.add(1) as usize);
    result.push(1.0);
    for i in (0..order).rev() {
        let bit = (index >> i) & 1;
        let fill = if bit == 0 { -1.0 } else { 1.0 };
        result.push(fill);
    }
    result
}
