//!
//! MapRange maps a value from one range to a second range.
//!
//! For integer types it doesn't escalate to floats, but
//! uses the next bigger integer type.
//!
//! It works better than integer divide + scale up, as it
//! truly fills the full target range. Especially an end-point
//! in the source range maps to an end-point in the target range.
//!
//! This is implemented for all combinations of source type /
//! target type except i128 and u128.
//!

#![no_std]

/// Bounded numeric operations.
///
/// Useful for taking steps within a range.
///
/// When used with a reversed range it steps
/// in the reversed direction.
///
pub trait RangeOp
where
    Self: Sized,
{
    type Step;

    /// Addition. Bounded to min/max.
    fn add_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self;

    /// Subtraction. Bounded to min/max.
    fn sub_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self;
}

/// Map ranges to ranges.
pub trait MapRange<O>
where
    Self: Sized,
{
    /// Map from a source range to a target range.
    ///
    /// Returns None if self is out of bounds for range.
    /// Can also map reversed ranges.
    fn map_range(self, range: (Self, Self), o_range: (O, O)) -> Option<O>;
}

// -------------------------------------------------------------
// -------------------------------------------------------------

macro_rules! u_range_op {
    ($value_ty:ty, $step_ty:ty) => {
        impl RangeOp for $value_ty {
            type Step = $step_ty;

            #[inline(always)]
            fn add_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self {
                self.saturating_add(delta).clamp(bounds.0, bounds.1)
            }

            #[inline(always)]
            fn sub_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self {
                self.saturating_sub(delta).clamp(bounds.0, bounds.1)
            }
        }
    };
}

macro_rules! i_range_op {
    ($value_ty:ty, $step_ty:ty) => {
        impl RangeOp for $value_ty {
            type Step = $step_ty;

            #[inline(always)]
            fn add_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self {
                self.saturating_add_unsigned(delta)
                    .clamp(bounds.0, bounds.1)
            }

            #[inline(always)]
            fn sub_clamp(self, delta: Self::Step, bounds: (Self, Self)) -> Self {
                self.saturating_sub_unsigned(delta)
                    .clamp(bounds.0, bounds.1)
            }
        }
    };
}

u_range_op!(u8, u8);
u_range_op!(u16, u16);
u_range_op!(u32, u32);
u_range_op!(u64, u64);
u_range_op!(u128, u128);
u_range_op!(usize, usize);
i_range_op!(i8, u8);
i_range_op!(i16, u16);
i_range_op!(i32, u32);
i_range_op!(i64, u64);
i_range_op!(i128, u128);
i_range_op!(isize, usize);

impl RangeOp for f32 {
    type Step = f32;

    #[inline(always)]
    fn add_clamp(self, delta: Self, bounds: (Self, Self)) -> Self {
        (self + delta).clamp(bounds.0, bounds.1)
    }

    #[inline(always)]
    fn sub_clamp(self, delta: Self, bounds: (Self, Self)) -> Self {
        (self - delta).clamp(bounds.0, bounds.1)
    }
}

impl RangeOp for f64 {
    type Step = f64;

    #[inline(always)]
    fn add_clamp(self, delta: Self, bounds: (Self, Self)) -> Self {
        (self + delta).clamp(bounds.0, bounds.1)
    }

    #[inline(always)]
    fn sub_clamp(self, delta: Self, bounds: (Self, Self)) -> Self {
        (self - delta).clamp(bounds.0, bounds.1)
    }
}

// -------------------------------------------------------------
// unsigned -> unsigned
// -------------------------------------------------------------

macro_rules! uu_map_range {
    ($src_ty:ty, $calc_ty:ty, $tgt_ty:ty) => {
        impl MapRange<$tgt_ty> for $src_ty {
            #[inline(never)]
            fn map_range(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> Option<$tgt_ty> {
                if self < range.0 || self > range.1 {
                    return None;
                }

                // use absolute diff
                let d = (self - range.0) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                Some(o_range.0.wrapping_add(d2 as $tgt_ty))
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
            #[inline(never)]
            fn map_range(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> Option<$tgt_ty> {
                if self < range.0 || self > range.1 {
                    return None;
                }

                // use absolute diff
                let d = (self.abs_diff(range.0)) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1.abs_diff(range.0)) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                Some(o_range.0.wrapping_add(d2 as $tgt_ty))
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
            #[inline(never)]
            fn map_range(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> Option<$tgt_ty> {
                // oob
                if self < range.0 || self > range.1 {
                    return None;
                }

                // use absolute diff, signs are separate
                let d = (self - range.0) as $calc_ty;
                let delta_target = o_range.1.abs_diff(o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                Some(o_range.0.wrapping_add_unsigned(d2 as $utgt_ty))
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
            #[inline(never)]
            fn map_range(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> Option<$tgt_ty> {
                // oob
                if self < range.0 || self > range.1 {
                    return None;
                }

                // use absolute diff, signs are separate
                let d = (self.abs_diff(range.0)) as $calc_ty;
                let delta_target = o_range.1.abs_diff(o_range.0) as $calc_ty;
                let delta_source = (range.1.abs_diff(range.0)) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                Some(o_range.0.wrapping_add_unsigned(d2 as $utgt_ty))
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
            fn map_range(
                self,
                range: (Self, Self),
                o_range: ($tgt_ty, $tgt_ty),
            ) -> Option<$tgt_ty> {
                if self < range.0 || self > range.1 {
                    return None;
                }

                let d = (self - range.0) as $calc_ty;
                let delta_target = (o_range.1 - o_range.0) as $calc_ty;
                let delta_source = (range.1 - range.0) as $calc_ty;

                // scaled d
                let d2 = (d * delta_target) / delta_source;

                // shift d
                Some(o_range.0 + d2 as $tgt_ty)
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
f_map_range!(i8, f64, f64);
f_map_range!(i16, f64, f64);
f_map_range!(i32, f64, f64);
f_map_range!(i64, f64, f64);
f_map_range!(i128, f64, f64);
f_map_range!(usize, f64, f64);
f_map_range!(isize, f64, f64);

f_map_range!(u8, f32, f32);
f_map_range!(u16, f32, f32);
f_map_range!(u32, f32, f32);
f_map_range!(u64, f32, f32);
f_map_range!(u128, f32, f32);
f_map_range!(i8, f32, f32);
f_map_range!(i16, f32, f32);
f_map_range!(i32, f32, f32);
f_map_range!(i64, f32, f32);
f_map_range!(i128, f32, f32);
f_map_range!(usize, f32, f32);
f_map_range!(isize, f32, f32);
