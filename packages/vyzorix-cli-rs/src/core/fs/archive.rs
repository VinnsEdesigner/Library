use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

pub fn bundle_workspace(output: &str, target_dir: &str) -> anyhow::Result<()> {
    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", target_dir)?;
    tar.finish()?;
    Ok(())
}
