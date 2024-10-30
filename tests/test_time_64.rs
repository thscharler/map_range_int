use map_range_int::MapRange;
use std::hint::black_box;
use std::time::SystemTime;

const REPEAT: usize = 10000;
const REPEAT_F: f64 = 10000f64;

#[test]
fn test_map_ui() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let _x2: Option<i64> = black_box(x.map_range((0, 255), (-1000, 1000)));
        }
    }
    eprintln!(
        "u64->i64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_uu() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let _x2: Option<u64> = black_box(x.map_range((0, 255), (0, 2000)));
        }
    }
    eprintln!(
        "u64->u64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_ii() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i64 {
            let _x2: Option<i64> = black_box(x.map_range((-128, 127), (-1000, 1000)));
        }
    }
    eprintln!(
        "i64->i64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_iu() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i64 {
            let _x2: Option<u64> = black_box(x.map_range((-128, 127), (0, 2000)));
        }
    }
    eprintln!(
        "i64->u64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}