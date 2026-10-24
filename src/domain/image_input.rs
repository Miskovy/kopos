#[derive(Debug, Clone)]
struct ImageInput {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}
#[derive(Debug)]
enum ImageInputError {
    ZeroDimensions,
    DimensionsOverflow,
    PixelsCountMismatch,
}

impl ImageInput {
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Result<Self, ImageInputError> {
        if width == 0 || height == 0 {
            return Err(ImageInputError::ZeroDimensions);
        }
        let expected_pixel_count = width
            .checked_mul(height)
            .ok_or(ImageInputError::DimensionsOverflow)?;

        if pixels.len() != expected_pixel_count {
            return Err(ImageInputError::PixelsCountMismatch);
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width() || y >= self.height() {
            return None;
        }
        let index = y * self.width() + x;

        Some(self.pixels[index])
    }

    pub fn pixel_clamped(&self, x: isize, y: isize) -> u8 {
        let x = (x.max(0) as usize).min(self.width - 1);
        let y = (y.max(0) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_width() {
        let result = ImageInput::new(0, 250, vec![]);
        assert!(matches!(result, Err(ImageInputError::ZeroDimensions)));
    }

    #[test]
    fn rejects_zero_height() {
        let result = ImageInput::new(250, 0, vec![]);
        assert!(matches!(result, Err(ImageInputError::ZeroDimensions)));
    }

    #[test]
    fn prevent_overflow() {
        let result = ImageInput::new(usize::MAX, 2, vec![]);
        assert!(matches!(result, Err(ImageInputError::DimensionsOverflow)));
    }

    #[test]
    fn prevent_incorrect_pixel_count() {
        let result = ImageInput::new(2, 2, vec![]);
        assert!(matches!(result, Err(ImageInputError::PixelsCountMismatch)));
    }

    #[test]
    fn accepts_valid_image() {
        let image =
            ImageInput::new(3, 3, vec![9; 9]).expect("valid dimensions and pixels should succeed");

        assert_eq!(image.width(), 3);
        assert_eq!(image.height(), 3);
        assert_eq!(image.pixels(), &[9; 9]);
    }

    #[test]
    fn accepts_valid_pixels() {
        let image = ImageInput::new(3, 2, vec![10, 20, 30, 40, 50, 60])
            .expect("The test image should be valid");
        assert_eq!(image.pixel(0, 0), Some(10));
        assert_eq!(image.pixel(2, 1), Some(60));
        assert_eq!(image.pixel(3, 0), None);
        assert_eq!(image.pixel(0, 2), None);
    }

    #[test]
    fn test_clamp_pixel() {
        let image = ImageInput::new(1, 1, vec![10]).expect("The test image should be valid");

        assert_eq!(image.pixel_clamped(0, 0), 10);
        assert_eq!(image.pixel_clamped(-1, -1), 10);
        assert_eq!(image.pixel_clamped(99, 99), 10);
        assert_eq!(image.pixel_clamped(isize::MIN, isize::MAX), 10);
    }

    #[test]
    fn clamps_pixel_coordinates_to_image_edges() {
        let image = ImageInput::new(3, 2, vec![10, 20, 30, 40, 50, 60])
            .expect("The test image should be valid");

        assert_eq!(image.pixel_clamped(1, 1), 50);
        assert_eq!(image.pixel_clamped(-1, 0), 10);
        assert_eq!(image.pixel_clamped(3, 0), 30);
        assert_eq!(image.pixel_clamped(0, -1), 10);
        assert_eq!(image.pixel_clamped(0, 2), 40);
        assert_eq!(image.pixel_clamped(-1, -1), 10);
        assert_eq!(image.pixel_clamped(99, 99), 60);
        assert_eq!(image.pixel_clamped(isize::MIN, isize::MAX), 40);
        assert_eq!(image.pixel_clamped(isize::MAX, isize::MIN), 30);
    }
}
