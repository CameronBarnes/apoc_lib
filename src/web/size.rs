use std::process::Command;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;

/// Gets the size of an rsync resource in bytes
///
/// # Errors
///
/// Returns an error if it fails to parse the size from the regex output
pub fn rsync(path: &str, exclude_str: &str) -> Result<u64> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new("Total file size: (.*?) bytes").expect("Regex expression should be valid")
    });
    let output = Command::new("rsync")
        .args([
            "--info=stats2",
            "-r",
            &format!("--exclude=*{exclude_str}"),
            path,
        ])
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    let size = RE
        .captures(&output)
        .ok_or_else(|| anyhow!("Failed to capture total file size from rsync output"))?
        .get(1).ok_or_else(|| anyhow!("Less than the expected at least two matches captured by regex"))?
        .as_str();
    Ok(size.replace(',', "").parse()?)
}
