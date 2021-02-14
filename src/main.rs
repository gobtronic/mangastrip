use crate::image::Device;
use clap::Clap;
use iced::{Application, Settings};
use opt::Opt;
use std::{fs::create_dir, path::Path, process::exit};

mod gui;
mod image;
mod opt;

#[cfg(test)]
mod tests;

fn main() {
    let _ = gui::Input::run(Settings::default());
    /*let opt: Opt = Opt::parse();
    let in_path = Path::new(&opt.input);
    if !in_path.exists() {
        println!();
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        let _ = writeln!(t, "Input doesn't exist");
        t.reset().unwrap();
        exit(1)
    }

    let out_path = match opt.output_dir {
        Some(p) => p,
        None => {
            // TODO: This is ugly
            if in_path.is_file() {
                let mut p = in_path.parent().unwrap().to_str().unwrap().to_string();
                if p == "" {
                    p = "./".to_string();
                }

                p
            } else {
                in_path.to_str().unwrap().to_string()
            }
        }
    };
    let out_path = Path::new(&out_path);
    if out_path.is_file() {
        println!();
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        let _ = writeln!(t, "Output should be a directory");
        t.reset().unwrap();
        exit(1)
    }
    if !out_path.exists() {
        println!("{:?}", out_path);
        if create_dir(out_path).is_err() {
            println!();
            let mut t = term::stdout().unwrap();
            t.fg(term::color::RED).unwrap();
            let _ = writeln!(
                t,
                "An error occured while trying to create output directory"
            );
            t.reset().unwrap();
            exit(1)
        }
    }

    let device = Device::Custom(opt.width, opt.height);

    let _ = process::process(in_path, out_path, &device);*/
}

mod process {
    use crate::image::{self, Device};
    use std::{fs, io, path::Path};

    /// Process a directory or file at the specified path.
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

    /// Process a file at the specified path.
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
}
