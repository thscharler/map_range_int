use map_range_int::MapRange;
use std::hint::black_box;
use std::time::SystemTime;

const REPEAT: usize = 10000;
const REPEAT_F: f64 = 10000f64;

#[test]
fn test_map_ui() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u32 {
            let _x2: Option<i32> = black_box(x.map_range((0, 255), (-1000, 1000)));
        }
    }
    eprintln!(
        "u32->i32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_uu() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u32 {
            let _x2: Option<u32> = black_box(x.map_range((0, 255), (0, 2000)));
        }
    }
    eprintln!(
        "u32->u32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_ii() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i32 {
            let _x2: Option<i32> = black_box(x.map_range((-128, 127), (-1000, 1000)));
        }
    }
    eprintln!(
        "i32->i32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_iu() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in -128..127i32 {
            let _x2: Option<u32> = black_box(x.map_range((-128, 127), (0, 2000)));
        }
    }
    eprintln!(
        "i32->u32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}
