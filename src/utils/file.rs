use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn open_file(name: String) -> anyhow::Result<String> {
    let mut dir = env::current_dir()?;
    let mut file: Option<File> = None;
    dir.push(&name);
    if dir.exists() {
        file = Some(File::open(dir)?);
    };
    let try_abs = Path::new(&name);
    if try_abs.exists() {
        file = Some(File::open(try_abs)?);
    }
    let mut buff = String::new();
    match file {
        Some(mut f) => {
            f.read_to_string(&mut buff);
        }
        None => {}
    };
    Ok(buff)
}
