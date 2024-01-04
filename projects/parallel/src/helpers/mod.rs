use std::fs::create_dir;
use std::path::{Path, PathBuf};
use nalgebra::{Complex, DMatrix, Dyn, OVector};

pub fn copy_vec_ref<T: Copy>(vec: Vec<&T>) -> Vec<T> {
    vec.into_iter().copied().collect()
}

pub fn polynomial_eigenvalues(input: &[f32]) -> OVector<Complex<f32>, Dyn>  {
    let dim = input.len();
    let mat: DMatrix<f32> = DMatrix::from_fn(dim, dim, |r, c| {
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

pub fn find_target_dir<P: AsRef<Path>>(here: P) -> std::io::Result<PathBuf> {
    let here = here.as_ref();
    let here = if here.is_file() {
        match here.parent() {
            Some(s) => {s.to_path_buf()}
            None => panic!("No parent directory found")
        }

    } else {
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
            None => break
        }
    }
    let new = here.join("target");
    create_dir(&new)?;
    Ok(new)
}