use crate::image::Device;
use clap::Clap;
use opt::Opt;
use std::{fs::create_dir, path::Path, process::exit};

mod image;
mod input;
mod logger;
mod opt;

#[cfg(test)]
mod tests;

fn main() {
    let opt: Opt = Opt::parse();

    let in_path = Path::new(&opt.input);
    if !in_path.exists() {
        logger::println("Input doesn't exist", logger::Type::Error);
        exit(1)
    }

    if let Some(out_path) = build_out_path(&opt) {
        if out_path.is_file() {
            logger::println("Output should be a directory", logger::Type::Error);
            exit(1)
        }
        if !out_path.exists() && create_dir(out_path).is_err() {
            logger::println(
                "An error occured while trying to create output directory",
                logger::Type::Error,
            );
            exit(1)
        }

        let device = Device::Custom(opt.width, opt.height);
        let _ = input::process(in_path, out_path, &device);
    } else {
        logger::println(
            "An error occured while trying to build output path",
            logger::Type::Error,
        );
        exit(1)
    }
}

/// Build output `Path` from `Opt` args.
/// 
/// ## Case scenarios
/// #### 1. User specified `--output-dir`
/// return `opt.output_dir`
/// #### 2. User didn't specify and `--input` is a **file** 
/// return `opt.input.parent()`
/// #### 3. User didn't specify and `--input` is a **dir**  
/// return `opt.input`
/// #### 4. Some error occured while parsing paths    
/// return `None`
fn build_out_path(opt: &opt::Opt) -> Option<&Path> {
    let out_path = match opt.output_dir {
        Some(ref p) => {
            println!("on devrait être là frero");
            p
        },
        None => {
            let in_path = Path::new(&opt.input);
            if in_path.is_file() {
                return match in_path.parent() {
                    Some(p) => {
                        let p = match p.to_str()? {
                            "" => "./",
                            p => p,
                        };

                        return Some(Path::new(p));
                    }
                    None => None,
                };
            } else {
                return Some(in_path);
            }
        }
    };

    Some(Path::new(out_path))
}
