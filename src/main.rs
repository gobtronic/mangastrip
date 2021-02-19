use crate::image::Device;
use clap::Clap;
use opt::Opt;
use std::{fs::create_dir, path::Path, process::exit};

mod image;
mod io;
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

    if let Some(out_path) = io::output::build(&opt) {
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
        let _ = io::input::process(in_path, out_path, &device);
    } else {
        logger::println(
            "An error occured while trying to build output path",
            logger::Type::Error,
        );
        exit(1)
    }
}
