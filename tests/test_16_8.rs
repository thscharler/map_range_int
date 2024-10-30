use map_range_int::MapRange;

// unsigned->unsigned mapping

#[test]
fn test_uu() {
    for x in (0..=65535u16).step_by(113) {
        let x2: Option<u8> = x.map_range((0, 65535), (0, 255));
        // eprintln!("{:?} -> {:?}", x, x2);
        assert_eq!(Some((x / 257) as u8), x2);
    }

    let x2: Option<u8> = 65535u16.map_range((0, 65535), (0, 255));
    // eprintln!("{:?} -> {:?}", 65535u16, x2);
    assert_eq!(Some((65535u16 / 257) as u8), x2);
}

// unsigned->signed mapping

#[test]
fn test_ui() {
    for x in 0..=255u8 {
        let x2: Option<i16> = x.map_range((0, 255), (-32768, 32767));
        assert_eq!(
            Some((-32768i16).saturating_add_unsigned(x as u16 * 257)),
            x2
        );
    }
}

// signed->unsigned mapping

#[test]
fn test_iu() {
    for x in -128..=127i8 {
        let x2: Option<u16> = x.map_range((-128, 127), (0, 65535));
        assert_eq!(
            Some(0u16.saturating_add((-128i8).abs_diff(x) as u16 * 257)),
            x2
        );
    }
}

// signed->signed mapping

#[test]
fn test_ii() {
    for x in -128..=127i8 {
        let x2: Option<i8> = x.map_range((-128, 127), (-128, 127));
        assert_eq!(
            Some((-128i8).saturating_add_unsigned((-128i8).abs_diff(x))),
            x2
        );
    }
}
