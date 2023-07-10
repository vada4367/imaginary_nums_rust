const U: i32 = 0; // 0 - pyth trips, 1 - mand.

mod complex;
use crate::complex::imaginary::Imaginary;
use crate::complex::Complex;

// Pyth const's
const N: i32 = 10;

// Mand const's
const X: f64 = -0.5505;
const Y: f64 = -0.53875;
const C: char = '␁'; // Color


fn main() {
    if U == 0 {
        use crate::complex::pyth;
        println!("{:#?}", pyth::pyth_trips(N));
    }

    if U == 1 {
        use crate::complex::mand;
        use std::time::{Duration, Instant};

        let frame_time = Duration::from_millis(0);

        let mut scale: f64 = 0.05; // Scale of coord network
        let mut frame = Instant::now();

        loop {
            // loop scale
            if Instant::now() - frame > frame_time {
                scale /= 1.01;

                if let Some((tx, ty)) = term_size::dimensions() {
                    let coord: mand::Coord = mand::Coord::init(
                        // coord network
                        X - tx as f64 / 2. * scale * 2.,
                        Y - ty as f64 / 2. * scale * 5.,
                        X + tx as f64 / 2. * scale * 2.,
                        Y + ty as f64 / 2. * scale * 5.,
                        tx as i32,
                        ty as i32,
                    );
                    let bool_coord: Vec<Vec<bool>> = coord.mand(); // Which dots is mond
                    mand::print_mand(&bool_coord, C);

                    frame = Instant::now();
                } else {
                    eprintln!("65 string ERROR");
                    break;
                }
            }
        }
    }
}
