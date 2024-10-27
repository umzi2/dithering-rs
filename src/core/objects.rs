pub struct Offset {
    pub x: isize,
    pub y: isize,
    pub divisor: u8,
    pub dividend: u8,
    pub weight: f32,
}
impl Offset {
    pub fn get_other(&self) -> (isize, isize, u8, u8) {
        (self.x, self.y, self.divisor, self.dividend)
    }
    pub fn get_f32(&self) -> (&isize, &isize, &f32) {
        (&self.x, &self.y, &self.weight)
    }
}
