use map_range_int::MapRange;
use std::hint::black_box;
use std::time::SystemTime;

const REPEAT: usize = 10000;
const REPEAT_F: f64 = 10000f64;

#[test]
fn test_map_f64() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let x = x as f64;
            let _x2: Option<f64> = black_box(x.map_range((0., 255.), (-1000., 1000.)));
        }
    }
    eprintln!(
        "f64->f64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_f32() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let x = x as f64;
            let _x2: Option<f32> = black_box(x.map_range((0., 255.), (0., 2000.)));
        }
    }
    eprintln!(
        "f64->f32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_u32() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let x = x as f64;
            let _x2: Option<u32> = black_box(x.map_range((0., 255.), (0, 2000)));
        }
    }
    eprintln!(
        "f64->u32 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_u32_f64() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u32 {
            let _x2: Option<f64> = black_box(x.map_range((0, 255), (0., 2000.)));
        }
    }
    eprintln!(
        "u32->f64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_u16() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u64 {
            let x = x as f64;
            let _x2: Option<u16> = black_box(x.map_range((0., 255.), (0, 2000)));
        }
    }
    eprintln!(
        "f64->u16 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}

#[test]
fn test_map_u16_f64() {
    let t0 = SystemTime::now();
    for _ in 0..REPEAT {
        for x in 0..=255u16 {
            let _x2: Option<f64> = black_box(x.map_range((0, 255), (0., 2000.)));
        }
    }
    eprintln!(
        "u16->f64 {:#?}ns",
        (t0.elapsed().unwrap().as_secs_f64() / (255f64 * REPEAT_F)) * 1e9f64
    );
}
