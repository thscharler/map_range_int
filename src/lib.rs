#![no_std]
#![doc = include_str!("../readme.md")]

/// Map ranges to ranges.
pub trait MapRange<Out>
where
    Self: PartialOrd + Sized,
    Out: PartialOrd,
{
    /// Map from a source range to a target range.
    ///
    /// Returns None if self is out of bounds for range.
    /// Can also map reversed ranges.
    #[inline(always)]
    fn map_range(self, range: (Self, Self), o_range: (Out, Out)) -> Option<Out> {
        if self < range.0 || self > range.1 {
            return None;
        }
        if o_range.1 < o_range.0 {
            return None;
        }
        Some(self.map_range_unchecked(range, o_range))
    }

    /// Map from a source range to a target range.
    /// Does no bounds checks, but avoids division by zero for an empty
    /// target range.
    ///
    /// If the value is out of bounds the result will be wrong, as this
    /// uses wrapping_add.
    ///
    /// __Panic__
    ///
    /// It will panic for reversed ranges in some cases.
    fn map_range_unchecked(self, range: (Self, Self), o_range: (Out, Out)) -> Out;
}

// -------------------------------------------------------------
// unsigned -> unsigned
// -------------------------------------------------------------

macro_rules! uu_map_range {
    ($src_ty:ty, $calc_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(always)]
            fn map_range_unchecked(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> $tgt_ty {
                if range.0 == range.1 {
                    return o_range.0;
                }

                // use absolute diff
                let d = (self - range.0) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                o_range.0.wrapping_add(d2 as $tgt_ty)
            }
        }
    };
}

uu_map_range!(u8, u16, u8);
uu_map_range!(u8, u32, u16);
uu_map_range!(u8, u64, u32);
uu_map_range!(u8, u128, u64);
uu_map_range!(u8, u128, usize);

uu_map_range!(u16, u32, u8);
uu_map_range!(u16, u32, u16);
uu_map_range!(u16, u64, u32);
uu_map_range!(u16, u128, u64);
uu_map_range!(u16, u128, usize);

uu_map_range!(u32, u64, u8);
uu_map_range!(u32, u64, u16);
uu_map_range!(u32, u64, u32);
uu_map_range!(u32, u128, u64);
uu_map_range!(u32, u128, usize);

uu_map_range!(u64, u128, u8);
uu_map_range!(u64, u128, u16);
uu_map_range!(u64, u128, u32);
uu_map_range!(u64, u128, u64);
uu_map_range!(u64, u128, usize);

uu_map_range!(usize, u128, u8);
uu_map_range!(usize, u128, u16);
uu_map_range!(usize, u128, u32);
uu_map_range!(usize, u128, u64);
uu_map_range!(usize, u128, usize);

// -------------------------------------------------------------
// signed -> unsigned
// -------------------------------------------------------------

macro_rules! iu_map_range {
    ($src_ty:ty, $calc_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(always)]
            fn map_range_unchecked(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> $tgt_ty {
                if range.0 == range.1 {
                    return o_range.0;
                }

                // use absolute diff
                let d = (self.abs_diff(range.0)) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1.abs_diff(range.0)) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                o_range.0.wrapping_add(d2 as $tgt_ty)
            }
        }
    };
}

iu_map_range!(i8, u16, u8);
iu_map_range!(i8, u32, u16);
iu_map_range!(i8, u64, u32);
iu_map_range!(i8, u128, u64);
iu_map_range!(i8, u128, usize);

iu_map_range!(i16, u32, u8);
iu_map_range!(i16, u32, u16);
iu_map_range!(i16, u64, u32);
iu_map_range!(i16, u128, u64);
iu_map_range!(i16, u128, usize);

iu_map_range!(i32, u64, u8);
iu_map_range!(i32, u64, u16);
iu_map_range!(i32, u64, u32);
iu_map_range!(i32, u128, u64);
iu_map_range!(i32, u128, usize);

iu_map_range!(i64, u128, u8);
iu_map_range!(i64, u128, u16);
iu_map_range!(i64, u128, u32);
iu_map_range!(i64, u128, u64);
iu_map_range!(i64, u128, usize);

iu_map_range!(isize, u128, u8);
iu_map_range!(isize, u128, u16);
iu_map_range!(isize, u128, u32);
iu_map_range!(isize, u128, u64);
iu_map_range!(isize, u128, usize);

// -------------------------------------------------------------
// unsigned -> signed
// -------------------------------------------------------------

macro_rules! ui_map_range {
    ($src_ty:ty, $calc_ty:ty, $utgt_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(always)]
            fn map_range_unchecked(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> $tgt_ty {
                if range.0 == range.1 {
                    return o_range.0;
                }

                // use absolute diff, signs are separate
                let d = (self - range.0) as $calc_ty;
                let delta_target = o_range.1.abs_diff(o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                o_range.0.wrapping_add_unsigned(d2 as $utgt_ty)
            }
        }
    };
}

ui_map_range!(u8, u16, u8, i8);
ui_map_range!(u8, u32, u16, i16);
ui_map_range!(u8, u64, u32, i32);
ui_map_range!(u8, u128, u64, i64);
ui_map_range!(u8, u128, usize, isize);

ui_map_range!(u16, u32, u8, i8);
ui_map_range!(u16, u32, u16, i16);
ui_map_range!(u16, u64, u32, i32);
ui_map_range!(u16, u128, u64, i64);
ui_map_range!(u16, u128, usize, isize);

ui_map_range!(u32, u64, u8, i8);
ui_map_range!(u32, u64, u16, i16);
ui_map_range!(u32, u64, u32, i32);
ui_map_range!(u32, u128, u64, i64);
ui_map_range!(u32, u128, usize, isize);

ui_map_range!(u64, u128, u8, i8);
ui_map_range!(u64, u128, u16, i16);
ui_map_range!(u64, u128, u32, i32);
ui_map_range!(u64, u128, u64, i64);
ui_map_range!(u64, u128, usize, isize);

ui_map_range!(usize, u128, u8, i8);
ui_map_range!(usize, u128, u16, i16);
ui_map_range!(usize, u128, u32, i32);
ui_map_range!(usize, u128, u64, i64);
ui_map_range!(usize, u128, usize, isize);

// -------------------------------------------------------------
// signed -> signed
// -------------------------------------------------------------

macro_rules! ii_map_range {
    ($src_ty:ty, $calc_ty:ty, $utgt_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(always)]
            fn map_range_unchecked(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> $tgt_ty {
                // oob
                if range.0 == range.1 {
                    return o_range.0;
                }

                // use absolute diff, signs are separate
                let d = (self.abs_diff(range.0)) as $calc_ty;
                let delta_target = o_range.1.abs_diff(o_range.0) as $calc_ty;
                let delta_source = (range.1.abs_diff(range.0)) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                o_range.0.wrapping_add_unsigned(d2 as $utgt_ty)
            }
        }
    };
}

ii_map_range!(i8, u16, u8, i8);
ii_map_range!(i8, u32, u16, i16);
ii_map_range!(i8, u64, u32, i32);
ii_map_range!(i8, u128, u64, i64);
ii_map_range!(i8, u128, usize, isize);

ii_map_range!(i16, u32, u8, i8);
ii_map_range!(i16, u32, u16, i16);
ii_map_range!(i16, u64, u32, i32);
ii_map_range!(i16, u128, u64, i64);
ii_map_range!(i16, u128, usize, isize);

ii_map_range!(i32, u64, u8, i8);
ii_map_range!(i32, u64, u16, i16);
ii_map_range!(i32, u64, u32, i32);
ii_map_range!(i32, u128, u64, i64);
ii_map_range!(i32, u128, usize, isize);

ii_map_range!(i64, u128, u8, i8);
ii_map_range!(i64, u128, u16, i16);
ii_map_range!(i64, u128, u32, i32);
ii_map_range!(i64, u128, u64, i64);
ii_map_range!(i64, u128, usize, isize);

ii_map_range!(isize, u128, u8, i8);
ii_map_range!(isize, u128, u16, i16);
ii_map_range!(isize, u128, u32, i32);
ii_map_range!(isize, u128, u64, i64);
ii_map_range!(isize, u128, usize, isize);

// -------------------------------------------------------------
// -------------------------------------------------------------

macro_rules! f_map_range {
    ($src_ty:ty, $calc_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(always)]
            fn map_range_unchecked(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> $tgt_ty {
                if range.0 == range.1 {
                    return o_range.0;
                }

                let d = (self - range.0) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                o_range.0 + d2 as $tgt_ty
            }
        }
    };
}

f_map_range!(f64, f64, f64);
f_map_range!(f64, f32, f32);
f_map_range!(f64, f64, u8);
f_map_range!(f64, f64, u16);
f_map_range!(f64, f64, u32);
f_map_range!(f64, f64, u64);
f_map_range!(f64, f64, u128);
f_map_range!(f64, f64, usize);
f_map_range!(f64, f64, i8);
f_map_range!(f64, f64, i16);
f_map_range!(f64, f64, i32);
f_map_range!(f64, f64, i64);
f_map_range!(f64, f64, i128);
f_map_range!(f64, f64, isize);

f_map_range!(f32, f64, f64);
f_map_range!(f32, f32, f32);
f_map_range!(f32, f32, u8);
f_map_range!(f32, f32, u16);
f_map_range!(f32, f32, u32);
f_map_range!(f32, f32, u64);
f_map_range!(f32, f32, u128);
f_map_range!(f32, f32, usize);
f_map_range!(f32, f32, i8);
f_map_range!(f32, f32, i16);
f_map_range!(f32, f32, i32);
f_map_range!(f32, f32, i64);
f_map_range!(f32, f32, i128);
f_map_range!(f32, f32, isize);

f_map_range!(u8, f64, f64);
f_map_range!(u16, f64, f64);
f_map_range!(u32, f64, f64);
f_map_range!(u64, f64, f64);
f_map_range!(u128, f64, f64);
f_map_range!(usize, f64, f64);
f_map_range!(i8, f64, f64);
f_map_range!(i16, f64, f64);
f_map_range!(i32, f64, f64);
f_map_range!(i64, f64, f64);
f_map_range!(i128, f64, f64);
f_map_range!(isize, f64, f64);

f_map_range!(u8, f32, f32);
f_map_range!(u16, f32, f32);
f_map_range!(u32, f32, f32);
f_map_range!(u64, f32, f32);
f_map_range!(u128, f32, f32);
f_map_range!(usize, f32, f32);
f_map_range!(i8, f32, f32);
f_map_range!(i16, f32, f32);
f_map_range!(i32, f32, f32);
f_map_range!(i64, f32, f32);
f_map_range!(i128, f32, f32);
f_map_range!(isize, f32, f32);
