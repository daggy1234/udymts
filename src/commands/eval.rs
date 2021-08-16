use crate::utils::decoder;
use crate::utils::file;
use pyo3::prelude::*;
use pyo3::{py_run, PyCell, PyObjectProtocol};

#[pyclass]
struct Udymts {
    name: String,
    decoded: String,
}

pub fn wrap_str_eval(code: String) -> anyhow::Result<()> {
    println!("Starting eval. Acquiring GIL!");
    println!("=============================");
    Python::with_gil(|py| {
        let lang = Udymts {
            name: "udymts".to_string(),
            decoded: "python".to_string(),
        };
        let udymts = PyCell::new(py, lang).unwrap();
        py_run!(py, udymts, &code);
    });
    println!("=============================");
    println!("Completed complete execution!");
    Ok(())
}

pub fn run_with_text(data: String) -> anyhow::Result<()> {
    let decoded = decoder::decode_cipher(data);
    wrap_str_eval(decoded)?;
    Ok(())
}

pub fn run_with_file(data: String) -> anyhow::Result<()> {
    let fc = file::open_file(data)?;
    if fc.len() < 1 {
        println!("Could not find file in directory, or absolute path");
        return Ok(());
    }
    let decoded = decoder::decode_cipher(fc);
    wrap_str_eval(decoded)?;
    Ok(())
}
