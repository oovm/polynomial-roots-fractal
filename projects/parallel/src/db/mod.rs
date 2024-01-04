use std::convert::TryInto;
use std::path::Path;
use itertools::Itertools;
use sled::{Db};
use crate::find_target_dir;
use sled::IVec;
use self::point::{Point, Counter};

mod point;
mod littlewood;


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
