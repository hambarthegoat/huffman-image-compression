use std::{error::Error, ffi::OsStr, path::PathBuf};

pub fn ensure_valid_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg") => Ok(()),
        _ => Err("Input file must have a .jpg or .jpeg extension".into()),
    }
}

pub fn ensure_himg_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("himg") => Ok(()),
        _ => Err("Input file must have a .himg extension".into()),
    }
}

pub fn ensure_output_extension(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg") => Ok(()),
        _ => Err("Output file must have a .jpg or .jpeg extension".into()),
    }
}
