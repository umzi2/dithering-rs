use crate::core::svec::SVec;
use crate::core::traits::NotFloat;
use num::Bounded;
use num_traits::cast::NumCast;
use num_traits::PrimInt;
pub trait QuantizeTrait<T> {
    fn get_new_pix(&self, pix: T) -> T;
    fn quantize_img(&self, img: &mut SVec<T>);
}
fn get_delta<T: Bounded + NumCast>(colors_per_channel: T) -> f32 {
    let channels: f32 = NumCast::from(colors_per_channel).expect("Failed to convert");
    if channels < 2.0 {
        panic!("colors_per_channel must be greater than 1");
    }
    let max_value: f32 = NumCast::from(T::max_value()).expect("Failed to convert max_value");
    (max_value) / (channels - 1.0)
}
fn get_delta_f32<T: Bounded + NumCast>(colors_per_channel: T) -> f32 {
    let channels: f32 = NumCast::from(colors_per_channel).expect("Failed to convert");
    if channels < 2.0 {
        panic!("colors_per_channel must be greater than 1");
    }
    (1.0) / (channels - 1.0)
}

pub struct Quantize {
    delta: f32,
}
impl<T> QuantizeTrait<T> for Quantize
where
    T: PrimInt + NumCast + NotFloat,
{
    fn get_new_pix(&self, pix: T) -> T {
        let pix_f32: f32 = NumCast::from(pix).expect("Failed to convert pix to f32");
        NumCast::from(((pix_f32 / self.delta).floor() * self.delta).round())
            .expect("Failed to convert f32 to T")
    }
    fn quantize_img(&self, image: &mut SVec<T>) {
        let mut_vec = image.get_data_mut();
        for value in mut_vec.iter_mut() {
            *value = self.get_new_pix(*value);
        }
    }
}

impl QuantizeTrait<f32> for Quantize {
    fn get_new_pix(&self, pix: f32) -> f32 {
        (pix / self.delta).round() * self.delta
    }
    fn quantize_img(&self, image: &mut SVec<f32>) {
        let mut_vec = image.get_data_mut();
        for value in mut_vec.iter_mut() {
            *value = self.get_new_pix(*value);
        }
    }
}
impl QuantizeTrait<f64> for Quantize {
    fn get_new_pix(&self, pix: f64) -> f64 {
        (pix / self.delta as f64).round() * self.delta as f64
    }
    fn quantize_img(&self, image: &mut SVec<f64>) {
        let mut_vec = image.get_data_mut();
        for value in mut_vec.iter_mut() {
            *value = self.get_new_pix(*value);
        }
    }
}
impl Quantize {
    pub fn new<T: Bounded + NumCast + 'static>(colors_per_channel: T) -> Self {
        let delta = if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>()
            || std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>()
        {
            get_delta_f32(colors_per_channel)
        } else {
            get_delta(colors_per_channel)
        };
        Quantize { delta }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::svec::{SVec, Shape};

    #[test]
    fn test_quantize_u8() {
        let mut img = SVec::new(
            Shape::new(5, 5, Some(3)),
            vec![
                0u8, 255, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128, 128, 128u8, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128u8, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
            ],
        );
        let e = Quantize::new(8u8);
        e.quantize_img(&mut img);
        println!("{:?}", img)
    }
}
