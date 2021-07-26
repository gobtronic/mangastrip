use std::ffi::OsStr;

use self::input::FileFormat;

pub mod input {
    use super::ExtFormat;
    use crate::{
        image::{self, Device},
        logger,
    };
    use std::{fs::{self, create_dir, remove_dir, remove_dir_all}, io, path::{Path, PathBuf}};

    pub enum FileFormat {
        Image,
        Archive,
        Unsupported,
    }

    /// Start processing the file or directory at the specified path for the specified `Device`.
    pub fn process(in_path: &PathBuf, out_path: &PathBuf, device: &Device) -> io::Result<()> {
        if in_path.is_dir() {
            for entry in fs::read_dir(in_path)? {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        process_file(&entry.path(), &out_path, device);
                    }
                }
            }
        } else {
            process_file(in_path, out_path, device);
        }

        Ok(())
    }

    /// Process the supported file at the specified path for the specified `Device`.
    fn process_file(f_path: &PathBuf, out_path: &PathBuf, device: &Device) {
        if let Some(f_ext) = f_path.extension() {
            match f_ext.file_format() {
                FileFormat::Image => process_image(f_path, out_path, device),
                FileFormat::Archive => process_archive(f_path, out_path, device),
                FileFormat::Unsupported => false,
            };
        }
    }

    /// Process an image at the specified path for the specified `Device`.
    /// Returns a `bool` indicating if the image has been processed successfully.
    fn process_image(f_path: &PathBuf, out_path: &PathBuf, device: &Device) -> bool {
        if !f_path.is_file() {
            return false;
        }

        match image::process(&f_path, device) {
            Ok(img) => {
                // TODO: This is ugly
                let filename = f_path.file_name().unwrap().to_str().unwrap();
                let out_path = out_path.join("opt_".to_owned() + filename);
                match img.save(out_path) {
                    Ok(_) => {
                        logger::println("Optimized image saved!", logger::Type::Success);
                        true
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Process an archive at the specified path for the specified `Device`.
    /// Returns a `bool` indicating if the archive has been processed successfully.
    fn process_archive(f_path: &PathBuf, out_path: &PathBuf, device: &Device) -> bool {
        if !f_path.is_file() {
            return false;
        }

        let archive = fs::File::open(&f_path).unwrap();
        let mut zip = zip::ZipArchive::new(archive).unwrap();

        let tmp_p = Path::new(out_path).join("tmp/");
        let _ = create_dir(tmp_p.clone());
        if zip.extract(tmp_p.clone()).is_ok() {
            process(&tmp_p, out_path, &device);
            let _ = remove_dir_all(tmp_p);
        } else {
            logger::println(
                "An error occured while extracting the provided input file!",
                logger::Type::Error,
            );
            return false;
        }

        true
    }
}

pub mod output {
    use crate::opt;
    use std::path::Path;

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
    pub fn build(opt: &opt::Opt) -> Option<std::path::PathBuf> {
        let out_path = match opt.output_dir {
            Some(ref p) => p,
            None => {
                let in_path = Path::new(&opt.input);
                if in_path.is_file() {
                    return match in_path.parent() {
                        Some(p) => {
                            let p = match p.to_str()? {
                                "" => "./",
                                p => p,
                            };

                            let path = Path::new(p);
                            return Some(path.join("mangastrip_output/"));
                        }
                        None => None,
                    };
                } else {
                    return Some(in_path.join("mangastrip_output/"));
                }
            }
        };

        Some(Path::new(out_path).join("mangastrip_output/"))
    }
}

trait ExtFormat {
    fn is_image(&self) -> bool;
    fn is_archive(&self) -> bool;
    fn file_format(&self) -> FileFormat;
}

impl ExtFormat for OsStr {
    fn is_image(&self) -> bool {
        self == "png" || self == "jpg"
    }

    fn is_archive(&self) -> bool {
        self == "zip" || self == "rar" || self == "cbz" || self == "cbr"
    }

    fn file_format(&self) -> FileFormat {
        if self.is_image() {
            return FileFormat::Image;
        } else if self.is_archive() {
            return FileFormat::Archive;
        }

        FileFormat::Unsupported
    }
}
