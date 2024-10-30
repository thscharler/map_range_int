use map_range_int::RangeOp;

#[test]
fn test_step() {
    let v = 0u8;

    assert_eq!(v.add_clamp(2, (50, 100)), 50);

    let v = 0f64;
    assert_eq!(v.add_clamp(2f64, (50f64, 100f64)), 50f64);

    let v = 0f32;
    assert_eq!(v.add_clamp(2f32, (50f32, 100f32)), 50f32);
}
