use map_range_int::MapRange;
use std::hint::black_box;
use std::time::SystemTime;

const REPEAT: usize = 10000;
const REPEAT_F: f64 = 10000f64;

#[test]
fn test_map_ui_unchecked() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u32 {
            let _x2: i16 = black_box(x.map_range_unchecked((0, 255), (-1000, 1000)));
        }
    }
    eprintln!(
        "u32->i16 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_uu_unchecked() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u32 {
            let _x2: u16 = black_box(x.map_range_unchecked((0, 255), (0, 2000)));
        }
    }
    eprintln!(
        "u32->u16 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_ii_unchecked() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i32 {
            let _x2: i16 = black_box(x.map_range_unchecked((-128, 127), (-1000, 1000)));
        }
    }
    eprintln!(
        "i32->i16 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_iu_unchecked() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i32 {
            let _x2: u16 = black_box(x.map_range_unchecked((-128, 127), (0, 2000)));
        }
    }
    eprintln!(
        "i32->u16 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}