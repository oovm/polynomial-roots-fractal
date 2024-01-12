use super::*;

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
    pub fn small_view(&self) {
        print!("viewing littlewood table {}:\n", self.order);
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        for data in self.table.iter().values() {
            match data {
                Ok(value) => {
                    // let point = u32::from_be_bytes(key.as_ref().try_into().unwrap());
                    let [x1, x2, x3, x4, y1, y2, y3, y4] = value.as_ref().try_into().unwrap();
                    let x = f32::from_be_bytes([x1, x2, x3, x4]);
                    let y = f32::from_be_bytes([y1, y2, y3, y4]);
                    if y < 1.0 / MAX_RESOLUTION as f32 {
                        continue;
                    }

                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
                Err(o) => {
                    eprintln!("Fatal error: {}", o);
                }
            }
        }
        println!("max_x: {}, max_y: {}", max_x, max_y);
    }
}

impl LittlewoodTable {
    pub fn solve_points_on_disk(&self) -> std::io::Result<()> {
        self.table.clear()?;
        let tasks = 2u32.pow(self.order);
        println!("Calculating littlewood rank {} with {} tasks", self.order, tasks);
        let bar = create_progress_bar(tasks as u64);
        (0..tasks).into_par_iter().for_each(|index| {
            self.update_point(index).unwrap();
            bar.inc(1);
        });
        bar.finish();
        Ok(())
    }
    pub fn solve_points_in_memory(&self) -> Result<PathBuf, EvaluateError> {
        let tasks = 2u64.pow(self.order);
        println!("Calculating littlewood rank {} with {} tasks", self.order, tasks);
        let bar = create_progress_bar(tasks);
        let target = find_target_dir(Path::new(env!("CARGO_MANIFEST_DIR")))?;
        let path = target.join("PolynomialRoots").join("littlewood").join(format!("complex_{}.wxf", self.order));
        let mut file = File::create(&path)?;
        file.write(&[56, 58, 194, 51, 1])?;
        let solutions: Vec<_> = (0..tasks).into_par_iter().map(|id| self.aberth_solver2(id, &bar)).flatten_iter().collect();
        file.write_varint(solutions.len() / 8)?;
        file.write_all(&solutions)?;
        bar.finish();
        Ok(path.canonicalize()?)
    }
    pub fn update_point(&self, index: u32) -> std::io::Result<()> {
        let equation = make_equation(index as u64, self.order as u64);
        let mut solver = aberth_solver();
        let mut batch = Batch::default();
        for root in solver.find_roots(&equation).iter() {
            if root.re < 0.0 || root.im < 0.0 {
                continue;
            }
            let [x1, x2, x3, x4] = (root.re as f32).to_be_bytes();
            let [y1, y2, y3, y4] = (root.im as f32).to_be_bytes();
            batch.insert(&index.to_be_bytes(), &[x1, x2, x3, x4, y1, y2, y3, y4])
        }
        self.table.apply_batch(batch)?;
        Ok(())
    }
    #[allow(dead_code)]
    fn aberth_solver(&self, task_id: u64, progress: &ProgressBar) -> Result<Vec<WolframValue>, EvaluateError> {
        let solutions = aberth_solver()
            .find_roots(&make_equation(task_id, self.order as u64))
            .iter()
            .filter(|root| 0.0 <= root.re && root.re <= 2.0 && 0.0 <= root.im && root.im <= 1.5)
            .map(|root| WolframValue::list(vec![root.re.to_wolfram(), root.im.to_wolfram()]))
            .collect();
        progress.inc(1);
        Ok(solutions)
    }
    /// Reduce throughput pressure and improve performance by 10%
    #[allow(dead_code)]
    fn aberth_solver2(&self, task_id: u64, progress: &ProgressBar) -> Vec<u8> {
        let data: Vec<_> = aberth_solver()
            .find_roots(&make_equation(task_id, self.order as u64))
            .into_iter()
            .filter(|root| 0.0 <= root.re && 0.0 <= root.im)
            .map(|root| {
                let [x1, x2, x3, x4] = (root.re as f32).to_le_bytes();
                let [y1, y2, y3, y4] = (root.im as f32).to_le_bytes();
                [x1, x2, x3, x4, y1, y2, y3, y4]
            })
            .flatten()
            .collect();
        progress.inc(1);
        data
    }
}

impl LittlewoodTable {
    pub fn update_pixel(&self, x: f32, y: f32) -> std::io::Result<()> {
        if x < 0.0 || y < 0.0 {
            return Ok(());
        }
        let x = (x * MAX_RESOLUTION as f32) as u32;
        let y = (y * MAX_RESOLUTION as f32) as u32;
        let point = Point { x, y };
        let mut counter = match self.table.get(&point)? {
            Some(s) => Counter::from(s),
            None => Counter::default(),
        };
        counter.n += 1;
        self.table.insert(point, counter)?;
        Ok(())
    }
    pub fn evaluate_matrix(&self) -> std::io::Result<()> {
        self.table.clear()?;
        let tasks = 2u32.pow(self.order);
        println!("Calculating littlewood rank {} with {} tasks", self.order, tasks);
        let bar = create_progress_bar(tasks as u64);
        (0..tasks).into_par_iter().for_each(|i| {
            let equation = make_equation(i as u64, self.order as u64);
            // let roots = polynomial_eigenvalues(&equation);
            for root in aberth_solver().find_roots(&equation).iter() {
                self.update_pixel(root.re as f32, root.im as f32).unwrap();
            }
            bar.inc(1);
        });
        bar.finish();
        Ok(())
    }

    // slow 10%
    #[allow(dead_code)]
    fn eigen_solver(&self, task_id: u64, progress: &ProgressBar) -> Result<Vec<WolframValue>, EvaluateError> {
        let equation = make_equation(task_id, self.order as u64);
        let solutions = polynomial_eigenvalues(&equation)
            .iter()
            .map(|root| WolframValue::list(vec![root.re.to_wolfram(), root.im.to_wolfram()]))
            .collect();
        progress.inc(1);
        Ok(solutions)
    }
}

fn make_equation(index: u64, order: u64) -> Vec<f64> {
    let mut result = Vec::with_capacity(order.add(1) as usize);
    result.push(1.0);
    for i in (0..order).rev() {
        let bit = (index >> i) & 1;
        let fill = if bit == 0 { -1.0 } else { 1.0 };
        result.push(fill);
    }
    result
}

fn aberth_solver() -> AberthSolver<f64> {
    let mut solver = AberthSolver::new();
    solver.epsilon = 0.01 / MAX_RESOLUTION as f64;
    solver.max_iterations = 24;
    solver
}

fn polynomial_eigenvalues(input: &[f64]) -> OVector<Complex<f64>, Dyn> {
    let dim = input.len();
    let mat: DMatrix<f64> = DMatrix::from_fn(dim, dim, |r, c| {
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
