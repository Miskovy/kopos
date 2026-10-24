use crate::domain::box_filter::box_filter_5x5;
use crate::domain::image_input::{ImageInput, ImageInputError};

/// Applies a serial 5×5 box blur to an image that is already grayscale.
///
/// `pixels` contains one byte per pixel in row-major order: all pixels in the
/// first row, followed by all pixels in the second row, and so on. Its length
/// must equal `width * height`, and both dimensions must be positive.
///
/// Each output pixel is the average of 25 samples centered on its position.
/// Samples outside the image use the nearest edge pixel. Integer division rounds
/// the average down. The returned buffer has the same layout and dimensions.
///
/// This function takes ownership of `pixels` and allocates a separate output
/// buffer. It runs synchronously on the calling thread and does not use workers.
///
/// # Errors
///
/// Returns [`ImageInputError::ZeroDimensions`] if either dimension is zero,
/// [`ImageInputError::DimensionsOverflow`] if their product overflows `usize`,
/// or [`ImageInputError::PixelsCountMismatch`] if the buffer length is incorrect.
/// Input validation completes before filtering starts.
///
/// # Examples
///
/// ```
/// use kopos::api::{ImageInputError, filter_grayscale};
///
/// let output = filter_grayscale(3, 2, vec![10, 20, 30, 40, 50, 60])?;
/// println!("Filtered pixels: {output:?}");
/// # Ok::<(), ImageInputError>(())
/// ```
pub fn filter_grayscale(
    width: usize,
    height: usize,
    pixels: Vec<u8>,
) -> Result<Vec<u8>, ImageInputError> {
    let image = ImageInput::new(width, height, pixels)?;
    Ok(box_filter_5x5(&image))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_valid_grayscale_image() {
        let output = filter_grayscale(3, 2, vec![10, 20, 30, 40, 50, 60])
            .expect("valid input should be filtered");

        assert_eq!(output, vec![28, 32, 36, 34, 38, 42]);
    }

    #[test]
    fn rejects_zero_dimensions() {
        for (width, height) in [(0, 2), (2, 0), (0, 0)] {
            let result = filter_grayscale(width, height, vec![]);

            assert!(matches!(result, Err(ImageInputError::ZeroDimensions)));
        }
    }

    #[test]
    fn rejects_incorrect_pixel_count() {
        for pixels in [vec![], vec![10, 20, 30], vec![10, 20, 30, 40, 50]] {
            let result = filter_grayscale(2, 2, pixels);

            assert!(matches!(result, Err(ImageInputError::PixelsCountMismatch)));
        }
    }

    #[test]
    fn rejects_dimension_overflow() {
        let result = filter_grayscale(usize::MAX, 2, vec![]);

        assert!(matches!(result, Err(ImageInputError::DimensionsOverflow)));
    }
}
