use super::*;

impl EvaluateCommand {
    pub fn run(self) -> Result<(), String> {
        let root = PolynomialRootsDatabase::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        match self.model.as_str() {
            "littlewood" => {
                let range = self.get_range()?;
                for rank in range {
                    root.littlewood_table(rank).unwrap().evaluate().unwrap()
                }
                Ok(())
            }
            _ => Err(format!("Unknown model {}", self.model)),
        }
    }

    pub fn get_range(&self) -> Result<RangeInclusive<u32>, String> {
        let split = self.orders.split(':').collect::<Vec<&str>>();
        match split.as_slice() {
            [index] => {
                let start = index.parse::<u32>().map_err(|_| "Invalid range parameter")?;
                Ok(start..=start)
            }
            [start, end] => {
                let start = start.parse::<u32>().map_err(|_| "Invalid range parameter")?;
                let end = end.parse::<u32>().map_err(|_| "Invalid range parameter")?;
                Ok(start..=end)
            }
            _ => Err("Invalid range parameter".to_string()),
        }
    }
}
