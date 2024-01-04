use self::point::{Counter, Point};
use crate::{find_target_dir, polynomial_eigenvalues};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::iter::{plumbing::Folder, IntoParallelIterator, ParallelIterator};
use sled::{Db, IVec};
use std::{convert::TryInto, path::Path};

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
