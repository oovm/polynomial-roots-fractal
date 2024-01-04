use super::*;
use std::sync::{Arc, Mutex};

pub struct LittlewoodTable {
    order: u32,
    table: sled::Tree,
}

impl PolynomialRootsDatabase {
    pub fn littlewood_table(&self, rank: u32) -> std::io::Result<LittlewoodTable> {
        let tb: sled::Tree = self.database.open_tree(format!("Littlewood-{}", rank).as_bytes())?;
        Ok(LittlewoodTable { order: rank, table: tb })
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
            Some(s) => Counter::from(s),
            None => Counter::default(),
        };
        counter.n += 1;
        self.table.insert(point, counter)?;
        Ok(())
    }
    pub fn evaluate(&self) -> std::io::Result<()> {
        self.table.clear()?;
        let tasks = 2u32.pow(self.order);
        println!("Calculating littlewood rank {} with {} tasks", self.order, tasks);
        let bar = {
            let bar = ProgressBar::new(tasks.add(1) as u64);
            bar.set_style(
                ProgressStyle::with_template("{bar:100.cyan/blue} [Time {elapsed_precise}, ETA {eta_precise}]").unwrap(),
            );
            bar
        };
        (0..tasks).into_par_iter().for_each(|i| {
            let mut solver = AberthSolver::new();
            solver.epsilon = 0.01 / HALF_RESOLUTION;
            solver.max_iterations = 16;
            let equation = make_equation(i as u64, self.order as u64);
            // let roots = polynomial_eigenvalues(&equation);
            for root in solver.find_roots(&equation).iter() {
                self.update(root.re, root.im).unwrap();
            }
            bar.inc(1);
        });
        bar.finish();
        Ok(())
    }

    pub fn evaluate_array(&self) -> Vec<WolframValue> {
        let tasks = 2u64.pow(self.order);
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(tasks as usize)));
        println!("Calculating littlewood rank {} with {} tasks", self.order, tasks);
        let bar = ProgressBar::new(tasks);
        bar.set_style(ProgressStyle::with_template("{bar:100.cyan/blue} [Time {elapsed_precise}, ETA {eta_precise}]").unwrap());
        (0..tasks).into_par_iter().for_each(|id| self.aberth_solver(id, &bar, buffer.clone()).unwrap());
        bar.finish();
        Arc::try_unwrap(buffer).unwrap().into_inner().unwrap()
    }
    #[allow(dead_code)]
    fn aberth_solver(
        &self,
        task_id: u64,
        progress: &ProgressBar,
        buffer: Arc<Mutex<Vec<WolframValue>>>,
    ) -> Result<(), EvaluateError> {
        let mut solver = AberthSolver::new();
        solver.epsilon = 0.1 / HALF_RESOLUTION;
        solver.max_iterations = 16;
        let equation = make_equation(task_id, self.order as u64);
        let mut lock = buffer.lock().unwrap();
        for root in solver.find_roots(&equation).iter() {
            lock.push(WolframValue::list(vec![root.im.to_wolfram(), root.re.to_wolfram()]));
        }
        progress.inc(1);
        Ok(())
    }

    // slow 10%
    #[allow(dead_code)]
    fn eigen_solver(&self, task_id: u64, progress: &ProgressBar) -> Result<Vec<WolframValue>, EvaluateError> {
        let equation = make_equation(task_id, self.order as u64);
        let solutions = polynomial_eigenvalues(&equation)
            .iter()
            .map(|root| WolframValue::list(vec![root.im.to_wolfram(), root.re.to_wolfram()]))
            .collect();
        progress.inc(1);
        Ok(solutions)
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

fn polynomial_eigenvalues(input: &[f32]) -> OVector<Complex<f32>, Dyn> {
    let dim = input.len();
    let mat: DMatrix<f32> = DMatrix::from_fn(dim, dim, |r, c| {
        if r == 0 {
            -input[c]
        }
        else if r == c + 1 {
            1.0
        }
        else {
            0.0
        }
    });
    mat.complex_eigenvalues()
}
