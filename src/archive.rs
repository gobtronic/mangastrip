/// Process an archive at the specified path for the specified `Device`,
/// returning a new archive optimized for the specified `Device` format.
pub fn process(path: &Path, device: &Device) -> Result<DynamicImage, ImageError> {
    println!();
    if let Some(p_str) = path.to_str() {
        logger::print(
            &format!("Starting {} conversion", p_str),
            logger::Type::Warning,
        );
    }

    let img = image::open(path)?;
    let img = borders::cut(&img);
    let img = size::resize(&img.0, img.1, device);
    Ok(img)
}