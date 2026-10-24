use super::image_input::ImageInput;

pub(crate) fn filter_pixel(image: &ImageInput, x: isize, y: isize) -> u8 {
    let mut sum: u32 = 0;
    for dy in -2..=2 {
        for dx in -2..=2 {
            let sample_x = x.saturating_add(dx);
            let sample_y = y.saturating_add(dy);

            let value = image.pixel_clamped(sample_x, sample_y);
            sum += u32::from(value);
        }
    }
    (sum / 25) as u8
}

pub(crate) fn box_filter_5x5(image: &ImageInput) -> Vec<u8> {
    let mut output = Vec::with_capacity(image.pixels().len());
    for j in 0..image.height() {
        for i in 0..image.width() {
            output.push(filter_pixel(image, i as isize, j as isize));
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_single_pixel() {
        let image = ImageInput::new(1, 1, vec![42]).expect("valid test image");
        assert_eq!(filter_pixel(&image, 0, 0), 42);
    }

    #[test]
    fn u8_overflow_test() {
        let image = ImageInput::new(3, 3, vec![255, 255, 255, 255, 255, 255, 255, 255, 255])
            .expect("valid test image");
        assert_eq!(filter_pixel(&image, 1, 1), 255);
    }

    #[test]
    fn edge_clamping_averaging() {
        let image = ImageInput::new(3, 2, vec![10, 20, 30, 40, 50, 60]).expect("valid test image");
        assert_eq!(filter_pixel(&image, 0, 0), 28);
    }

    #[test]
    fn filter_5x5() {
        let image = ImageInput::new(1, 1, vec![42]).expect("valid test image");
        assert_eq!(box_filter_5x5(&image), vec![42]);
    }

    #[test]
    fn filter_5x5_remain() {
        let image = ImageInput::new(2, 2, vec![255, 255, 255, 255]).expect("valid test image");
        assert_eq!(box_filter_5x5(&image), vec![255; 4]);
    }

    #[test]
    fn filters_image_without_changing_input() {
        let image = ImageInput::new(3, 2, vec![10, 20, 30, 40, 50, 60]).expect("valid test image");
        assert_eq!(box_filter_5x5(&image), vec![28, 32, 36, 34, 38, 42]);
        assert_eq!(image.pixels(), &[10, 20, 30, 40, 50, 60]);
    }
}
