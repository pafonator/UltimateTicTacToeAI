

pub struct Symmetry{
    pub x_increasing: bool,
    pub y_increasing: bool,
    pub invert_axes: bool
}

impl LeastSymmetry for UltraGrid {

    fn least_symmetry(s :&mut self) {
        /* There are 2 x 2 x 2 = 8 symmetries
            - Increasing / Decreasing X (start from left or right)
            - Increasing / Decreasing Y (start from top or bottom)
            - X then Y / Y then X
        */
        return;
    }
}