pub trait NotFloat {}
impl NotFloat for i8 {}
impl NotFloat for i16 {}
impl NotFloat for i32 {}
impl NotFloat for i64 {}
impl NotFloat for i128 {}
impl NotFloat for isize {}

// Целые типы без знака
impl NotFloat for u8 {}
impl NotFloat for u16 {}
impl NotFloat for u32 {}
impl NotFloat for u64 {}
impl NotFloat for u128 {}
impl NotFloat for usize {}
