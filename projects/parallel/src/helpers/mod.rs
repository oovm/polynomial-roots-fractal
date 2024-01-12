use crate::EvaluateError;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::create_dir,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};
pub fn copy_vec_ref<T: Copy>(vec: Vec<&T>) -> Vec<T> {
    vec.into_iter().copied().collect()
}

pub fn find_target_dir<P: AsRef<Path>>(here: P) -> std::io::Result<PathBuf> {
    let here = here.as_ref();
    let here = if here.is_file() {
        match here.parent() {
            Some(s) => s.to_path_buf(),
            None => panic!("No parent directory found"),
        }
    }
    else {
        here.to_path_buf()
    };
    let mut parent = here.clone();
    loop {
        for child in parent.read_dir()? {
            let child = child?;
            if child.file_name() == "target" {
                return Ok(child.path());
            }
        }
        parent = match parent.parent() {
            Some(parent) => parent.to_path_buf(),
            None => break,
        }
    }
    let new = here.join("target");
    create_dir(&new)?;
    Ok(new)
}

pub fn create_progress_bar(tasks: u64) -> ProgressBar {
    let bar = ProgressBar::new(tasks);
    bar.set_style(ProgressStyle::with_template("{bar:100.cyan/blue} [Time {elapsed_precise}, ETA {eta_precise}]").unwrap());
    bar
}

pub fn read_range(range: &str) -> Result<RangeInclusive<u32>, EvaluateError> {
    let split = range.split(':').collect::<Vec<&str>>();
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
