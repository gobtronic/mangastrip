use std::path::Path;

use image::{self, DynamicImage, ImageError};

pub enum Device {
    KoboForma,
    KindlePaperwhite,
    Custom(u32, u32),
}

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
    let img = size::resize(&img.0, img.1, Device::KoboForma);
    Ok(img)
}

mod borders {
    use super::bbox;
    use image::DynamicImage;

    pub enum BorderType {
        White,
        Black,
    }

    pub fn cut(img: &DynamicImage) -> (DynamicImage, BorderType) {
        println!("Calcutating bounds...");
        let bbox = bbox::bbox(&img, 50);
        println!("Removing borders...");
        (
            img.crop_imm(bbox.0.x, bbox.0.y, bbox.0.width, bbox.0.height),
            bbox.1,
        )
    }
}

mod size {
    use super::borders::BorderType;
    use super::Device;
    use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer};

    pub fn resize(img: &DynamicImage, b_type: BorderType, device: Device) -> DynamicImage {
        println!("Resizing to device form factor...");
        let img = img.resize(device.size().0, device.size().1, FilterType::CatmullRom);
        size_to_fit(&img, device, b_type)
    }

    fn size_to_fit(img: &DynamicImage, device: Device, b_type: BorderType) -> DynamicImage {
        let img_dim = img.dimensions();
        let mut sub_img =
            ImageBuffer::from_fn(device.size().0, device.size().1, |_, _| match b_type {
                BorderType::White => image::Rgb([255, 255, 255]),
                BorderType::Black => image::Rgb([0, 0, 0]),
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
        fn size(&self) -> (u32, u32) {
            match self {
                Device::KoboForma => (1440, 1920),
                Device::KindlePaperwhite => (1080, 1920),
                Device::Custom(w, h) => (*w, *h),
            }
        }
    }
}

mod bbox {
    use image::GrayImage;
    use image::{math::Rect, DynamicImage};
    use std::cmp;

    use super::borders::BorderType;

    struct Point {
        x: u32,
        y: u32,
    }

    pub struct Bbox {
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
    }

    impl Bbox {
        fn merge_small(&self, other: Bbox) -> Bbox {
            Bbox {
                left: cmp::max(self.left, other.left),
                top: cmp::max(self.top, other.top),
                right: cmp::min(self.right, other.right),
                bottom: cmp::min(self.bottom, other.bottom),
            }
        }

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

    pub fn bbox(img: &DynamicImage, tol: u8) -> (Rect, BorderType) {
        let lu_img: GrayImage = img.grayscale().into_luma8();
        let w_bbox = lu_bbox(&lu_img, true, tol);
        let b_bbox = lu_bbox(&lu_img, false, tol);

        println!("Determining border color...");
        let b_type = {
            let img_dim = lu_img.dimensions();
            match w_bbox.px_diff(img_dim) > b_bbox.px_diff(img_dim) {
                true => BorderType::White,
                false => BorderType::Black,
            }
        };
        (w_bbox.merge_small(b_bbox).into(), b_type)
    }

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
