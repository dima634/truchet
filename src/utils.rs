
#[inline]
pub fn flatten_2d_index(row: usize, col: usize, cols_count: usize) -> usize {
    return row * cols_count + col;
}
