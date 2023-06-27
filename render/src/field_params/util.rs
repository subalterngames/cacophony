pub(crate) const KV_PADDING: u32 = 2;

/// Returns half of the width, or slightly less than half.
/// The half-width is `width / 2` for odd numbers and `width / 2 - 1)` for even numbers.
pub(super) fn get_half_width(width: u32) -> usize {
    let mut half_width = width / 2;
    if width % 2 == 0 {
        half_width -= 1;
    }
    half_width as usize
}
