use crate::core::svec::SVec;
use num::{Bounded, Num};
use num_traits::cast::NumCast;
use num_traits::ToPrimitive;

fn get_delta<T: Bounded + NumCast + std::fmt::Display>(colors_per_channel: T) -> (f32, f32) {
    let channels: f32 = NumCast::from(colors_per_channel).expect("Failed to convert");
    if channels < 2.0 {
        panic!("colors_per_channel must be greater than 1");
    }
    let max_value: f32 = NumCast::from(T::max_value()).expect("Failed to convert max_value");
    ((max_value) / (channels - 1.0), max_value)
}
fn get_delta_f32<T: Bounded + NumCast + std::fmt::Display>(colors_per_channel: T) -> (f32, f32) {
    let channels: f32 = NumCast::from(colors_per_channel).expect("Failed to convert");
    if channels < 2.0 {
        panic!("colors_per_channel must be greater than 1");
    }
    let max_value: f32 = 1.0;
    ((max_value) / (channels - 1.0), max_value)
}
pub struct Quantize<T> {
    delta: f32,
    func: fn(f32) -> T,
}

fn convert_f32_to<T>(value: f32) -> T
where
    T: NumCast + Copy + ToPrimitive,
{
    NumCast::from(value.round()).expect("Failed to convert f32 to T")
}
fn return_f32<T>(value: f32) -> T
where
    T: NumCast + Copy + ToPrimitive,
{
    NumCast::from(value).expect("Failed to convert f32 to T")
}
impl<T: Bounded + NumCast + std::fmt::Display + Copy + 'static> Quantize<T> {
    pub fn new(colors_per_channel: T) -> Self {
        let (delta, max_value) = if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>() {
            get_delta_f32(colors_per_channel)
        } else {
            get_delta(colors_per_channel)
        };
        let func = if max_value == 1.0 {
            return_f32::<T>
        } else {
            convert_f32_to::<T>
        };

        Quantize { delta, func }
    }
    fn get_new_pix(&self, pix: T) -> T
    where
        T: Bounded + NumCast + Num + Copy,
    {
        let pix_f32: f32 = NumCast::from(pix).expect("Failed to convert pix to f32");
        let new_pix_f32 = (pix_f32 / self.delta).round() * self.delta;
        (self.func)(new_pix_f32)
    }
    pub fn quantize_img(&self, image: &mut SVec<T>)
    where
        T: Bounded + NumCast + Clone + Num + Copy,
    {
        let mut_vec = image.get_data_mut();
        for value in mut_vec.iter_mut() {
            *value = self.get_new_pix(*value);
        }
    }
}
