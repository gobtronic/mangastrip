mod borders {
    use crate::image::borders;
    use borders::BorderColor;
    use image::GenericImageView;
    use image::ImageError;

    #[test]
    fn cut_w() -> Result<(), ImageError> {
        let img = image::open("src/tests/cut_w.png")?;
        let img = borders::cut(&img);
        assert_eq!(img.0.dimensions(), (352, 348));
        assert!(img.1 == BorderColor::White);
        Ok(())
    }

    #[test]
    fn cut_b() -> Result<(), ImageError> {
        let img = image::open("src/tests/cut_b.png")?;
        let img = borders::cut(&img);
        assert_eq!(img.0.dimensions(), (345, 341));
        assert!(img.1 == BorderColor::Black);
        Ok(())
    }
}

mod bbox {
    use crate::image::bbox::Bbox;

    #[test]
    fn bbox_intersection() {
        let bbox_a = Bbox {
            left: 50,
            top: 80,
            right: 110,
            bottom: 140,
        };
        let bbox_b = Bbox {
            left: 35,
            top: 90,
            right: 50,
            bottom: 300,
        };
        let inter_bbox = Bbox::intersection(bbox_a, bbox_b);
        assert!(
            inter_bbox
                == Bbox {
                    left: 50,
                    top: 90,
                    right: 50,
                    bottom: 140
                }
        );
    }
}

mod size {
    use crate::image::{borders::BorderColor, size, Device};
    use image::GenericImageView;
    use image::ImageError;

    #[test]
    fn resize() -> Result<(), ImageError> {
        let img = image::open("src/tests/cut_w.png")?;
        let device = Device::KoboForma;
        let res_img = size::resize(&img, BorderColor::White, &device);
        assert_eq!(res_img.dimensions(), (device.size().0, device.size().1));
        Ok(())
    }
}
