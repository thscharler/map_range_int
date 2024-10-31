#[test]
fn test_boundaries() {
    use map_range_int::MapRange;

    // in range
    let r2 = 17u8.map_range((10, 20), (100, 200)).expect("in_range");
    assert_eq!(r2, 170u8);

    let r2 = 10u8.map_range((10, 20), (100, 200)).expect("in_range");
    assert_eq!(r2, 100u8);

    let r2 = 20u8.map_range((10, 20), (100, 200)).expect("in_range");
    assert_eq!(r2, 200u8);

    // out of range
    let r2: Option<u8> = 5u8.map_range((10, 20), (100, 200));
    assert_eq!(r2, None);

    // reversed bounds
    let r2: Option<u8> = 17u8.map_range(/**/ (20, 10) /**/, (100, 200));
    assert_eq!(r2, None);

    let r2: Option<u8> = 17u8.map_range((10, 20), /**/ (200, 100) /**/);
    assert_eq!(r2, None);

    // boundaries
    let r2 = 17u8
        .map_range((10, 20), /**/ (100, 100) /**/)
        .expect("in_range");
    assert_eq!(r2, 100u8);

    let r2 = 10u8.map_range((10, 10), (100, 200)).expect("in_range");
    assert_eq!(r2, 100u8);

    // signed
    let r2 = (-10i8).map_range((-100, 100), (0, 10)).expect("in_range");
    assert_eq!(r2, 4u8);

    let r2 = 90u8.map_range((0, 200), (-100, 100)).expect("in_range");
    assert_eq!(r2, -10i8);

    // float
    let r2 = 0.31f64.map_range((0., 1.), (0, 255)).expect("in_range");
    assert_eq!(r2, 79u8);

    let r2 = 81u8.map_range((0, 255), (0., 1.)).expect("in_range");
    assert_eq!(r2, 0.3176470588235294f64);
}
