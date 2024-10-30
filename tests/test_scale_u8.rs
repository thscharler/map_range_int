use map_range_int::MapRange;

#[test]
fn test_scale_10() {
    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, 25), (0, 250));

        if (0..=25u8).contains(&x) {
            assert_eq!(Some(x * 10), x2);
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_2() {
    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, 127), (0, 254));

        if (0..=127u8).contains(&x) {
            assert_eq!(Some(x * 2), x2);
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_3() {
    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, 85), (0, 255));

        if (0..=85u8).contains(&x) {
            assert_eq!(Some(x * 3), x2);
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_5() {
    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, 51), (0, 255));

        if (0..=51u8).contains(&x) {
            assert_eq!(Some(x * 5), x2);
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_7() {
    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, 36), (0, 255));

        if (0..=36u8).contains(&x) {
            // this doesn't say map_range() is wrong, my tests estimate is.
            if x >= 36 {
                assert_eq!((x * 7).abs_diff(x2.expect("value")), 3);
            } else if x >= 24 {
                assert_eq!((x * 7).abs_diff(x2.expect("value")), 2);
            } else if x >= 12 {
                assert_eq!((x * 7).abs_diff(x2.expect("value")), 1);
            } else {
                assert_eq!(Some(x * 7), x2);
            }
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_11() {
    let s = 11u8;
    let limit = 255 / s;

    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, limit), (0, 255));

        if (0..=limit).contains(&x) {
            // this doesn't say map_range() is wrong, my tests estimate is.
            if x >= 36 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 3);
            } else if x >= 23 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 2);
            } else if x >= 12 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 1);
            } else {
                assert_eq!(Some(x * s), x2);
            }
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_13() {
    let s = 13u8;
    let limit = 255 / s;

    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, limit), (0, 255));

        if (0..=limit).contains(&x) {
            // this doesn't say map_range() is wrong, my tests estimate is.
            if x >= 19 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 8);
            } else if x >= 17 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 7);
            } else if x >= 15 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 6);
            } else if x >= 12 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 5);
            } else if x >= 10 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 4);
            } else if x >= 8 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 3);
            } else if x >= 5 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 2);
            } else if x >= 3 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 1);
            } else {
                assert_eq!(Some(x * s), x2);
            }
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_17() {
    let s = 17u8;
    let limit = 255 / s;

    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, limit), (0, 255));

        if (0..=limit).contains(&x) {
            assert_eq!(Some(x * s), x2);
        } else {
            assert_eq!(None, x2);
        }
    }
}

#[test]
fn test_scale_19() {
    let s = 19u8;
    let limit = 255 / s;

    for x in 0..=255u8 {
        let x2: Option<u8> = x.map_range((0, limit), (0, 255));

        if (0..=limit).contains(&x) {
            // this doesn't say map_range() is wrong, my tests estimate is.
            if x >= 13 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 8);
            } else if x >= 12 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 7);
            } else if x >= 10 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 6);
            } else if x >= 9 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 5);
            } else if x >= 7 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 4);
            } else if x >= 5 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 3);
            } else if x >= 4 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 2);
            } else if x >= 2 {
                assert_eq!((x * s).abs_diff(x2.expect("value")), 1);
            } else {
                assert_eq!(Some(x * s), x2);
            }
        } else {
            assert_eq!(None, x2);
        }
    }
}
