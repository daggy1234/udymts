#[cfg(feature = "auto_updates")]
pub fn run() -> anyhow::Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("daggy1234")
        .repo_name("udymts")
        .bin_name("udymts")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

#[cfg(not(feature = "auto_updates"))]
pub fn run() -> anyhow::Result<()> {
    println!("Installed via package manager. Please update using the package manager used for installation");
    Ok(())
}

// #[cfg(not(feature = "auto_updates"))]
// pub fn run() -> anyhow::Result<()> {
//     println!("Installed via package manager. Please update using it");
//     Ok(())
// }
