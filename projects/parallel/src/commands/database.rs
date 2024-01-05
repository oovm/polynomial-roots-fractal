use super::*;

impl DatabaseCommand {
    pub fn run(self) -> Result<(), EvaluateError> {
        let target = find_target_dir(Path::new(env!("CARGO_MANIFEST_DIR")))?;
        let root = PolynomialRootsDatabase::new(Path::new(env!("CARGO_MANIFEST_DIR")))?;
        match self.model.as_str() {
            "littlewood" => {
                let range = read_range(&self.orders)?;
                create_dir_all(target.join("PolynomialRoots").join("littlewood"))?;
                for rank in range {
                    root.littlewood_table(rank)?.small_view();
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
