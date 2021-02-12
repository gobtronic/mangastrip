
use std::{io, fs, path::Path};

mod image;

fn main() {
    let _ = process(Path::new("input/"));
}

fn process(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    process_file(&entry.path());
                }
            }
        }
    } else if path.is_file() {
        process_file(path);
    }

    Ok(())
}

fn process_file(path: &Path) -> bool {
    if !path.is_file() {
        return false
    }

    let _ = fs::create_dir("output/");
    match image::process_image(path) {
        Ok(img) => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let output = format!("output/{}", filename);
            match img.save(output.clone()) {
                Ok(_) => {
                    println!("Image saved to {}", output);
                    return true
                }
                Err(_) => return false
            }
        }
        Err(_) => return false
    }
}