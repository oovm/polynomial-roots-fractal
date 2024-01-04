use std::convert::TryInto;
use sled::{CompareAndSwapResult, Db, InlineArray};
use crate::find_target_dir;

mod point;

use self::point::{Point, Counter};

const HALF_RESOLUTION: f32 = 300000.0 / 2.0;


pub struct PolynomialRootsDatabase {}


pub struct LittlewoodTable {
    table: sled::Tree,
}

impl LittlewoodTable {
    pub fn update(&self, x: f32, y: f32) -> std::io::Result<()> {
        if x < 0.0 || y < 0.0 {
            return Ok(());
        }
        let x = (x * HALF_RESOLUTION) as u32;
        let y = (y * HALF_RESOLUTION) as u32;
        let point = Point { x, y };
        //
        // match self.table.compare_and_swap(point, None, Some(Counter::default()))? {
        //     Ok(swap) => {
        //         swap.new_value
        //     }
        //     Err(e) => {}
        // }
        let mut counter = match self.table.get(&point)? {
            Some(s) => {


                Counter { n: u32::from_le_bytes(s.as_ref().try_into().unwrap()) }
            }
            None => { Counter::default() }
        };
        counter.n += 1;
        self.table.insert(point, counter)?;
        Ok(())
    }
}

#[test]
fn test() {
    let root = find_target_dir(env!("CARGO_MANIFEST_DIR")).unwrap();
    let db: Db = sled::open(root.join("PolynomialRoots")).unwrap();


    let littlewood: sled::Tree = db.open_tree(b"Littlewood1").unwrap();
    littlewood.insert(
        Point { x: 0, y: 0 },
        Counter { n: 0 },
    ).unwrap();
}
