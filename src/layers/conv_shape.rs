#[derive(Copy, Clone)]
pub struct ConvShape {
    pub depth: usize,
    pub height: usize,
    pub width: usize,
}

impl ConvShape {
    pub fn new(depth: usize, height: usize, width: usize) -> ConvShape {
        ConvShape {
            depth,
            height,
            width,
        }
    }
}