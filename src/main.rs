use crate::image::Device;
use clap::Clap;
use opt::Opt;
use std::{fs::create_dir, path::Path, process::exit};

mod opt;
mod input;
mod image;

#[cfg(test)]
mod tests;

fn main() {
    let opt: Opt = Opt::parse();
    let in_path = Path::new(&opt.input);
    
    if !in_path.exists() {
        println!();
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        let _ = writeln!(t, "Input doesn't exist");
        t.reset().unwrap();
        exit(1)
    }

    let out_path: Option<&str> = match opt.output_dir {
        Some(ref p) => Some(p),
        None => {
            if in_path.is_file() {
                match in_path.parent() {
                    Some(p) => {
                        let p = p.as_ref();
                        if let Some(p) = p {
                            if p == "" {
                                p = "./";
                                return p
                            }
                        }
                            

                        p
                    }
                    None => None
                }
            } else {
                in_path.to_str()
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
    let _ = input::process(in_path, out_path, &device);
}