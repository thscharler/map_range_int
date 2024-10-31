use map_range_int::MapRange;

#[test]
fn test_oob() {
    let x = 26u8;
    let x2: u8 = x.map_range_unchecked((0, 25), (0, 250));
    eprintln!("{} -> {}", x, x2);
    assert_eq!(x2, 4u8);
}

#[test]
fn test_oob2() {
    let x = 2f64;
    let x2: f64 = x.map_range_unchecked((0., 1.), (0., f64::MAX));
    eprintln!("{} -> {}", x, x2);
    assert!(x2.is_infinite());
}

#[test]
#[should_panic]
fn test_oob3() {
    let x = 26u8;
    let x2: u8 = x.map_range_unchecked((25, 0), (0, 250));
    // eprintln!("{} -> {}", x, x2);
    // will not panic in release mode, but wrap around.
    // this will induce the expected panic.
    assert_ne!(x2, 1u8);
}

#[test]
#[should_panic]
fn test_oob4() {
    let x = 26u8;
    let x2: u8 = x.map_range_unchecked((0, 25), (250, 0));
    // eprintln!("{} -> {}", x, x2);
    // will not panic in release mode, but wrap around.
    // this will induce the expected panic.
    assert_ne!(x2, 0u8);
}

#[test]
fn test_oob5() {
    let x = 29i8;
    let x2: i8 = x.map_range_unchecked((0, 25), (-100, 100));
    eprintln!("{} -> {}", x, x2);
    assert_eq!(x2, -124i8);
}
