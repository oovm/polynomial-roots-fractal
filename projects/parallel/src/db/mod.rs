use self::point::{Counter, Point};
use crate::{find_target_dir, EvaluateError};
use aberth::{AberthSolver, Complex};
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{DMatrix, Dyn, OVector};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sled::{Db, IVec};
use std::{convert::TryInto, iter::FromIterator, ops::Add, path::Path};
use wolfram_wxf::{ToWolfram, WolframValue};

mod littlewood;
mod point;

const HALF_RESOLUTION: f32 = 300000.0 / 2.0;

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
