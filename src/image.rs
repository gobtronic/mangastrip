use std::path::Path;

use image::{self, DynamicImage, ImageError};

pub fn process_image(path: &Path) -> Result<DynamicImage, ImageError> {
    println!();
    if let Some(p_str) = path.to_str() {
        println!("Starting {} conversion", p_str);
    }

    let mut img = image::open(path)?;
    println!("Calcutating bounds...");
    let bbox = bbox::bbox(&img,  50);
    println!("Removing margins...");
    img = img.crop(bbox.x, bbox.y, bbox.width, bbox.height);
    Ok(img)
}

mod bbox {
    use std::cmp;
    use image::{DynamicImage, math::Rect};
    use image::GrayImage;
    use imageproc::point::Point;

    struct Bbox {
        left: u32,
        top: u32,
        right: u32,
        bottom: u32
    }
    
    impl Bbox {
        fn merge_small(&self, other: Bbox) -> Bbox {
            Bbox {
                left: cmp::max(self.left, other.left),
                top: cmp::max(self.top, other.top),
                right: cmp::min(self.right, other.right),
                bottom: cmp::min(self.bottom, other.bottom)
            }
        }
    }
    
    impl Into<Rect> for Bbox {
        fn into(self) -> Rect {
            Rect {
                x: self.left,
                y: self.top,
                width: self.right - self.left,
                height: self.bottom - self.top
            }
        }
    }

    pub fn bbox(img: &DynamicImage, tol: u8) -> Rect {
        let lu_img: GrayImage = img.grayscale().into_luma8();
        let w_bbox = lu_bbox(&lu_img, true, tol);
        let b_bbox = lu_bbox(&lu_img, false, tol);
    
        w_bbox.merge_small(b_bbox).into()
    }
    
    fn lu_bbox(img: &GrayImage, white: bool, tol: u8) -> Bbox {
        let dim = img.dimensions();
        let mut coord = Bbox { left: 0, top: 0, right: 0, bottom: 0 };
    
        let mut i = Point::new(0, 0);
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
                i = Point::new(0, i.y + 1);
            } else {
                i = Point::new(i.x + 1, i.y);
            }
        }
    
        coord
    }
    
}

