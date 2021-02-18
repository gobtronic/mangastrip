use crate::image::{self, Device};
use std::{fs, io, path::Path};

/// Process a directory or file at the specified path for the specified `Device`.
pub fn process(in_path: &Path, out_path: &Path, device: &Device) -> io::Result<()> {
    if in_path.is_dir() {
        for entry in fs::read_dir(in_path)? {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    process_file(&entry.path(), out_path, device);
                }
            }
        }
    } else if in_path.is_file() {
        process_file(in_path, out_path, device);
    }

    Ok(())
}

/// Process a file at the specified path for the specified `Device`.
/// Returns a `bool` indicating if the file has been processed successfully.
fn process_file(f_path: &Path, out_path: &Path, device: &Device) -> bool {
    if !f_path.is_file() {
        return false;
    }

    let _ = fs::create_dir("output/");
    match image::process_image(f_path, device) {
        Ok(img) => {
            // TODO: This is ugly
            let filename = f_path.file_name().unwrap().to_str().unwrap();
            let out_path = out_path.join("opt_".to_owned() + filename);
            match img.save(out_path) {
                Ok(_) => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::GREEN).unwrap();
                    let _ = writeln!(t, "Optimized image saved!");
                    t.reset().unwrap();

                    true
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}