use crate::core::enums::ErrorDitheringEnum;
use crate::core::objects::Offset;
use crate::core::quantize::{Quantize, QuantizeTrait};
use crate::core::svec::SVec;
use crate::core::traits::NotFloat;
use num_traits::{NumCast, PrimInt, SaturatingAdd};

fn convert_err<T>(value: u8) -> T
where
    T: NumCast,
{
    NumCast::from(value).unwrap()
}

pub trait ErrorDitheringTrait<T> {
    fn dithering(&self, image: &mut SVec<T>);
}

pub struct ErrorDithering {
    quant: Quantize,
    offset: &'static [Offset],
}

impl ErrorDitheringTrait<f32> for ErrorDithering {
    fn dithering(&self, image: &mut SVec<f32>) {
        let (h, w, c) = image.shape();
        let data = image.get_data_mut();
        match c {
            Some(channel) => {
                for y in 0..h {
                    for x in 0..w {
                        for ch in 0..channel {
                            let index = (y * w + x) * channel + ch;
                            let pix = data[index];
                            let new_pix = self.quant.get_new_pix(pix);
                            unsafe {
                                *data.get_unchecked_mut(index) = new_pix;
                            }
                            let error_pix = pix - new_pix;
                            for offsets in self.offset {
                                let (dx, dy, weight) = offsets.get_f32();
                                let new_x = x as isize + dx;
                                let new_y = y as isize + dy;

                                if new_x >= 0
                                    && new_x < w as isize
                                    && new_y >= 0
                                    && new_y < h as isize
                                {
                                    let new_index = ((new_y * w as isize + new_x)
                                        * channel as isize
                                        + ch as isize)
                                        as usize;
                                    unsafe {
                                        *data.get_unchecked_mut(new_index) =
                                            (*data.get_unchecked(new_index) + error_pix * weight)
                                                .min(1.0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                for y in 0..h {
                    for x in 0..w {
                        let index = y * w + x;
                        let pix = data[index];
                        let new_pix = self.quant.get_new_pix(pix);
                        unsafe {
                            *data.get_unchecked_mut(index) = new_pix;
                        }
                        let error_pix = pix - new_pix;
                        for offsets in self.offset {
                            let (dx, dy, weight) = offsets.get_f32();
                            let new_x = x as isize + dx;
                            let new_y = y as isize + dy;

                            if new_x >= 0 && new_x < w as isize && new_y >= 0 && new_y < h as isize
                            {
                                let new_index = (new_y * w as isize + new_x) as usize;
                                unsafe {
                                    *data.get_unchecked_mut(new_index) =
                                        (*data.get_unchecked(new_index) + error_pix * weight)
                                            .min(1.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T> ErrorDitheringTrait<T> for ErrorDithering
where
    T: NotFloat + NumCast + SaturatingAdd + std::ops::Sub + PrimInt,
{
    fn dithering(&self, image: &mut SVec<T>) {
        let (h, w, c) = image.shape();
        let data = image.get_data_mut();
        let mut offsets: Vec<(isize, isize, T, T)> = Vec::new();
        for offset in self.offset {
            let (dx, dy, divisor, dividend) = offset.get_other();
            offsets.push((dx, dy, convert_err(divisor), convert_err(dividend)))
        }
        match c {
            Some(channel) => {
                for y in 0..h {
                    for x in 0..w {
                        for ch in 0..channel {
                            let index = (y * w + x) * channel + ch;
                            let pix = data[index];
                            let new_pix = self.quant.get_new_pix(pix);
                            data[index] = new_pix;
                            let error_pix = pix - new_pix;

                            for (dx, dy, divisor, dividend) in &offsets {
                                let new_x = x as isize + dx;
                                let new_y = y as isize + dy;

                                if new_x >= 0
                                    && new_x < w as isize
                                    && new_y >= 0
                                    && new_y < h as isize
                                {
                                    let new_index = ((new_y * w as isize + new_x)
                                        * channel as isize
                                        + ch as isize)
                                        as usize;
                                    unsafe {
                                        *data.get_unchecked_mut(new_index) = data
                                            .get_unchecked(new_index)
                                            .saturating_add(&(error_pix * *divisor / *dividend));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                for y in 0..h {
                    for x in 0..w {
                        let index = y * w + x;
                        let pix = data[index];
                        let new_pix = self.quant.get_new_pix(pix);
                        data[index] = new_pix;
                        let error_pix = pix - new_pix;

                        for (dx, dy, divisor, dividend) in &offsets {
                            let new_x = x as isize + dx;
                            let new_y = y as isize + dy;

                            // Проверка на границы
                            if new_x >= 0 && new_x < w as isize && new_y >= 0 && new_y < h as isize
                            {
                                let new_index = (new_y * w as isize + new_x) as usize;
                                unsafe {
                                    *data.get_unchecked_mut(new_index) = data
                                        .get_unchecked(new_index)
                                        .saturating_add(&(error_pix * *divisor / *dividend));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ErrorDithering {
    pub fn new(quant: Quantize, error_type: ErrorDitheringEnum) -> Self {
        ErrorDithering {
            quant,
            offset: error_type.distribution(),
        }
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
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                // 0.0,1.0,0.5,0.6,0.3,
                0u8, 255, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128, 128u8, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 123, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128u8, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
                128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
            ],
        );
        let e = ErrorDithering::new(Quantize::new(8u8), ErrorDitheringEnum::FloydSteinberg);
        e.dithering(&mut img);
        println!("{:?}", img)
    }
}
