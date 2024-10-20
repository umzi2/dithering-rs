#[derive(Debug)]
pub struct Shape {
    height: usize,
    width: usize,
    channels: Option<usize>,
}
#[derive(Debug)]
pub struct SVec<T> {
    shape: Shape,
    data: Vec<T>,
}
impl Shape {
    pub fn new(height: usize, width: usize, channels: Option<usize>) -> Self {
        Self {
            height,
            width,
            channels,
        }
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_channels(&self) -> Option<usize> {
        self.channels
    }
    pub fn get_shape(&self) -> (usize, usize, Option<usize>) {
        (self.height, self.width, self.channels)
    }
    pub fn get_ndims(&self) -> usize {
        if self.channels.is_some() {
            3
        } else {
            2
        }
    }
}
impl<T> SVec<T> {
    pub fn new(shape: Shape, data: Vec<T>) -> Self {
        SVec { shape, data }
    }
    pub fn shape(&self) -> (usize, usize, Option<usize>) {
        self.shape.get_shape()
    }
    pub fn get_data(&self) -> &[T] {
        self.data.as_slice()
    }
    pub fn get_data_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }
}
