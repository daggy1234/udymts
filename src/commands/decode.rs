use crate::utils::decoder;
use crate::utils::file;

pub fn run_with_text(data: String) -> anyhow::Result<()> {
    let decoded = decoder::encode_cipher(data);
    println!("decoded:\n{}", decoded);
    Ok(())
}

pub fn run_with_file(data: String) -> anyhow::Result<()> {
    let fc = file::open_file(data)?;
    if fc.len() < 1 {
        println!("Could not find file in directory, or absolute path");
        return Ok(());
    }
    let decoded = decoder::decode_cipher(fc);
    println!("{}", decoded);
    Ok(())
}
