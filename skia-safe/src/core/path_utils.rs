use crate::{prelude::*, Matrix, Paint, Path, Rect};
use skia_bindings as sb;

/// Returns the filled equivalent of the stroked path.
///
/// * `src` - [`Path`] read to create a filled version
/// * `paint` - uses settings for stroke cap, width, miter, join, and patheffect.
/// * `dst` - results are written to this builder.
/// * `cull_rect` - optional limit passed to [`PathEffect`]
/// * `ctm` - matrix to take into acount for increased precision (if it scales up).
///
/// Returns: `true` if the result can be filled, or `false` if it is a hairline (to be stroked).
pub fn fill_path_with_paint<'a>(
    src: &Path,
    paint: &Paint,
    dst: &mut Path,
    cull_rect: impl Into<Option<&'a Rect>>,
    ctm: impl Into<Option<Matrix>>,
) -> bool {
    let cull_rect: Option<&'a Rect> = cull_rect.into();
    let matrix = ctm.into().unwrap_or(Matrix::scale((1.0, 1.0)));

    unsafe {
        sb::C_PathUtils_FillPathWithPaint(
            src.native(),
            paint.native(),
            dst.native_mut(),
            cull_rect.native_ptr_or_null(),
            matrix.native(),
        )
    }
}
