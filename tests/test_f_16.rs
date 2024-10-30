use map_range_int::MapRange;

// #[test]
// fn test_f64_u16() {
//     for x in (0..=65535u16).step_by(113) {
//         let x = x as f64;
//         let x2: Option<u16> = x.map_range((0., 65535.), (0, 65535));
//         eprintln!("{:?} -> {:?}", x, x2);
//     }
//
//     let x2: Option<u16> = 65535f64.map_range((0., 65535.), (0, 65535));
//     eprintln!("{:?} -> {:?}", 65535u16, x2);
// }
//
// #[test]
// fn test_f64_u16_2() {
//     let mut x = 0f64;
//     loop {
//         let x2: Option<u16> = x.map_range((0., 1.), (0, 65535));
//         eprintln!("{:?} -> {:?}", x, x2);
//
//         x += 0.01f64;
//
//         if x > 1. {
//             break;
//         }
//     }
//
//     let x2: Option<u16> = 1f64.map_range((0., 1.), (0, 65535));
//     eprintln!("{:?} -> {:?}", 65535u16, x2);
// }
