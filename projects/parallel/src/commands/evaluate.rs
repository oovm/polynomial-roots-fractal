use super::*;
use crate::EvaluateError;
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use wolfram_wxf::WolframValue;

impl EvaluateCommand {
    pub fn run(self) -> Result<(), EvaluateError> {
        let target = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = PolynomialRootsDatabase::new(target).unwrap();
        match self.model.as_str() {
            "littlewood" => {
                let range = self.get_range()?;

                create_dir_all(target.join("PolynomialRoots").join("littlewood")).unwrap();

                for rank in range {
                    let roots = root.littlewood_table(rank).unwrap().evaluate_array();
                    let path = target.join("PolynomialRoots").join("littlewood").join(format!("complex_{}.wxf", rank));
                    let mut file = File::create(path).unwrap();
                    file.write_all(&WolframValue::list(roots).to_compressed()).unwrap();
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    pub fn get_range(&self) -> Result<RangeInclusive<u32>, EvaluateError> {
        let split = self.orders.split(':').collect::<Vec<&str>>();
        match split.as_slice() {
            [index] => {
                let start = index.parse::<u32>().unwrap();
                Ok(start..=start)
            }
            [start, end] => {
                let start = start.parse::<u32>().unwrap();
                let end = end.parse::<u32>().unwrap();
                Ok(start..=end)
            }
            _ => unreachable!(),
        }
    }
}
