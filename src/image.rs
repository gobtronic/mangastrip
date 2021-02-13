use std::path::Path;

use image::{self, DynamicImage, ImageError};

/// Representation of various ebook readers or tablets form factors.
///
/// The `Custom` variant allows you to make a generic device with custom width and height.
pub enum Device {
    KoboForma,
    KindlePaperwhite,
    Custom(u32, u32),
}

/// Process an image at the specified path,
/// returning a modified `DynamicImage` that fits the specified `Device` format.
pub fn process_image(path: &Path) -> Result<DynamicImage, ImageError> {
    println!();
    if let Some(p_str) = path.to_str() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::YELLOW).unwrap();
        writeln!(t, "Starting {} conversion", p_str).unwrap();
        t.reset().unwrap();
    }

    let img = image::open(path)?;
    let img = borders::cut(&img);
    let img = size::resize(&img.0, img.1, &Device::KoboForma);
    Ok(img)
}

pub mod borders {
    use super::bbox;
    use image::DynamicImage;

    #[derive(PartialEq)]
    /// Representation of the color of borders/margins you can find in mangas (black or white)
    pub enum BorderColor {
        White,
        Black,
    }

    /// Return a cropped image along with the `BorderColor` that has been detected and cropped-out.
    pub fn cut(img: &DynamicImage) -> (DynamicImage, BorderColor) {
        println!("Calcutating bounds...");
        let bbox = bbox::bbox(&img, 50);
        println!("Removing borders...");
        (
            img.crop_imm(bbox.0.x, bbox.0.y, bbox.0.width, bbox.0.height),
            bbox.1,
        )
    }
}

pub mod size {
    use super::borders::BorderColor;
    use super::Device;
    use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer};

    /// Return a resized image matching the specified `Device` size.
    /// Aspect ratio is preserved.
    ///
    /// ## Steps
    /// 1. Resize the image to the desired format by preserving the aspect ratio.
    /// 2. Create a canvas matching the exact `Device` size and fill it with the specified `BorderColor`.
    /// 3. Add the source image to the canvas by centering it as an overlay.
    pub fn resize(img: &DynamicImage, b_color: BorderColor, device: &Device) -> DynamicImage {
        println!("Resizing to device form factor...");
        let img = img.resize(device.size().0, device.size().1, FilterType::CatmullRom);
        let img_dim = img.dimensions();
        let mut sub_img =
            ImageBuffer::from_fn(device.size().0, device.size().1, |_, _| match b_color {
                BorderColor::White => image::Rgb([255, 255, 255]),
                BorderColor::Black => image::Rgb([0, 0, 0]),
            });

        image::imageops::overlay(
            &mut sub_img,
            &img.to_rgb8(),
            (device.size().0 - img_dim.0) / 2,
            (device.size().1 - img_dim.1) / 2,
        );
        DynamicImage::ImageRgb8(sub_img)
    }

    impl Device {
        /// The width and height of the device.
        pub fn size(&self) -> (u32, u32) {
            match self {
                Device::KoboForma => (1440, 1920),
                Device::KindlePaperwhite => (1080, 1920),
                Device::Custom(w, h) => (*w, *h),
            }
        }
    }
}

pub mod bbox {
    use image::GrayImage;
    use image::{math::Rect, DynamicImage};
    use std::cmp;

    use super::borders::BorderColor;

    /// It's a point.
    struct Point {
        x: u32,
        y: u32,
    }

    /// A struct representing a bounding box.
    #[derive(PartialEq)]
    pub struct Bbox {
        pub left: u32,
        pub top: u32,
        pub right: u32,
        pub bottom: u32,
    }

    impl Bbox {
        /// Return the intersection `Bbox` contained in both `a` and `b`.
        pub fn intersection(a: Bbox, b: Bbox) -> Bbox {
            Bbox {
                left: cmp::max(a.left, b.left),
                top: cmp::max(a.top, b.top),
                right: cmp::min(a.right, b.right),
                bottom: cmp::min(a.bottom, b.bottom),
            }
        }

        /// Return the difference in number of pixels between a `Bbox` and its container.
        fn px_diff(&self, container_dim: (u32, u32)) -> u32 {
            container_dim.0 * container_dim.1 - (self.right - self.left) * (self.bottom - self.top)
        }
    }

    impl Into<Rect> for Bbox {
        fn into(self) -> Rect {
            Rect {
                x: self.left,
                y: self.top,
                width: self.right - self.left,
                height: self.bottom - self.top,
            }
        }
    }

    /// Return a bounding box and the detected image `BorderColor`.
    /// The returned bounding box size should be the original image size without its borders.
    /// Adjust the border color tolerance value (I recommend between 0-50) to your liking.
    pub fn bbox(img: &DynamicImage, tol: u8) -> (Rect, BorderColor) {
        let lu_img: GrayImage = img.grayscale().into_luma8();
        let w_bbox = lu_bbox(&lu_img, true, tol);
        let b_bbox = lu_bbox(&lu_img, false, tol);

        println!("Determining border color...");
        let b_color = {
            let img_dim = lu_img.dimensions();
            match w_bbox.px_diff(img_dim) > b_bbox.px_diff(img_dim) {
                true => BorderColor::White,
                false => BorderColor::Black,
            }
        };
        (Bbox::intersection(w_bbox, b_bbox).into(), b_color)
    }

    /// Return a bounding box ignoring white or black borders.
    /// Adjust the border color tolerance value (I recommend between 0-50) to your liking.
    fn lu_bbox(img: &GrayImage, white: bool, tol: u8) -> Bbox {
        let dim = img.dimensions();
        let mut coord = Bbox {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        let mut i = Point { x: 0, y: 0 };
        for pi in img.pixels() {
            if white && pi.0[0] < 255 - tol || !white && pi.0[0] > tol {
                if coord.left == 0 || i.x < coord.left {
                    coord.left = i.x;
                }
                if coord.top == 0 || i.y < coord.top {
                    coord.top = i.y;
                }
                if i.x > coord.right {
                    coord.right = i.x
                }
                if i.y > coord.bottom {
                    coord.bottom = i.y;
                }
            }

            if i.x + 2 > dim.0 {
                i = Point { x: 0, y: i.y + 1 };
            } else {
                i = Point { x: i.x + 1, y: i.y };
            }
        }

        coord
    }
}
