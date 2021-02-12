use std::{fs, io, path::Path};

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
        return false;
    }

    let _ = fs::create_dir("output/");
    match image::process_image(path) {
        Ok(img) => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let output = format!("output/{}", filename);
            match img.save(output.clone()) {
                Ok(_) => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::GREEN).unwrap();
                    writeln!(t, "Image saved to {}", output).unwrap();
                    t.reset().unwrap();
                    
                    true
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
