use self::point::{Counter, Point};
use crate::{create_progress_bar, find_target_dir, EvaluateError};
use aberth::{AberthSolver, Complex};
use indicatif::ProgressBar;
use integer_encoding::VarIntWriter;
use nalgebra::{DMatrix, Dyn, OVector};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sled::{Batch, Db, IVec};
use std::{
    convert::TryInto,
    fs::File,
    io::Write,
    iter::FromIterator,
    ops::Add,
    path::{Path, PathBuf},
};
use wolfram_wxf::{ToWolfram, WolframValue};

mod littlewood;
mod point;

const MAX_RESOLUTION: u32 = 2u32.pow(15);

pub struct PolynomialRootsDatabase {
    database: Db,
}

impl PolynomialRootsDatabase {
    pub fn new(here: &Path) -> std::io::Result<Self> {
        let root = find_target_dir(here)?;
        let db: Db = sled::open(root.join("PolynomialRoots"))?;
        Ok(Self { database: db })
    }
}
