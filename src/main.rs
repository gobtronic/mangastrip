use image;
use image::math::Rect;
use image::GrayImage;
use imageproc::point::Point;

fn main() {
    process_image();
}

fn process_image() {
    let mut img = image::open("src/001.jpg").unwrap();
    let lu_img: GrayImage = img.grayscale().into_luma8();

    let bbox = bbox(&lu_img, true, 50);
    img = img.crop(bbox.x, bbox.y, bbox.width, bbox.height);
    let _ = img.save("src/001_cut.jpg");
}

fn bbox(img: &GrayImage, white: bool, tolerance: u8) -> Rect {
    let dim = img.dimensions();
    println!("{:?}", dim);
    let mut bbox = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    let mut i = Point::new(0, 0);
    for pi in img.pixels() {
        match white {
            true => {
                if pi.0[0] < 255 - tolerance {
                    if bbox.x != 0 && i.x < bbox.x {
                        bbox = Rect {
                            x: i.x,
                            y: bbox.y,
                            width: bbox.x - i.x + bbox.width,
                            height: bbox.height,
                        };
                    } else if bbox.x == 0 {
                        bbox = Rect {
                            x: i.x,
                            y: bbox.y,
                            width: bbox.width,
                            height: bbox.height,
                        };
                    }

                    if bbox.x > 0 {
                        if bbox.x + bbox.width < i.x {
                            bbox = Rect {
                                x: bbox.x,
                                y: bbox.y,
                                width: i.x - bbox.x,
                                height: bbox.height,
                            };
                        }
                    }

                    if bbox.y != 0 && i.y < bbox.y {
                        bbox = Rect {
                            x: bbox.x,
                            y: i.y,
                            width: bbox.width,
                            height: bbox.y - i.y + bbox.height,
                        };
                    } else if bbox.y == 0 {
                        bbox = Rect {
                            x: bbox.x,
                            y: i.y,
                            width: bbox.width,
                            height: bbox.height,
                        };
                    }

                    if bbox.y > 0 {
                        if bbox.y + bbox.height < i.y {
                            bbox = Rect {
                                x: bbox.x,
                                y: bbox.y,
                                width: bbox.width,
                                height: i.y - bbox.y,
                            };
                        }
                    }
                }
            }
            false => if pi.0[0] > 0 {},
        }

        if i.x + 2 > dim.0 {
            i = Point::new(0, i.y + 1);
        } else {
            i = Point::new(i.x + 1, i.y);
        }
    }

    bbox
}
