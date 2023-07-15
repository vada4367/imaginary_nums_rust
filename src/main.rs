const U: i32 = 1; // 0 - pyth trips, 1 - mand.

mod complex;
use crate::complex::imaginary::Imaginary;
use crate::complex::Complex;
use std::time::Duration;

// Pyth const's
const N: i32 = 10;

// Mand const's
const X: f64 = -0.5505;
const Y: f64 = -0.53875;

const HANDLES: i32 = 4;
const COLOR: char = '#';
const SPEED: f64 = 1.05;
const FRAME_TIME: Duration = Duration::from_millis(30);

#[tokio::main]
async fn main() {
    if U == 0 {
        use crate::complex::pyth;
        println!("{:#?}", pyth::pyth_trips(N));
    }

    if U == 1 {
        use crate::complex::mand;
        use std::time::Instant;

        let mut scale: f64 = 0.03; // Scale of coord network
        let mut frame = Instant::now();

        // loop scale
        loop {
            if Instant::now() - frame > FRAME_TIME {
                scale /= SPEED;

                let (tx, ty): (usize, usize) = term_size::dimensions().unwrap();

                let coord: mand::Coord = mand::Coord::init(
                    X - tx as f64 / 2. * scale * 2.,
                    Y - ty as f64 / 2. * scale * 5.,
                    X + tx as f64 / 2. * scale * 2.,
                    Y + ty as f64 / 2. * scale * 5.,
                    tx as i32,
                    ty as i32,
                );

                let bool_coord: Vec<bool> = coord.clone().mand(HANDLES).await; // Which dots is mond

                mand::print_mand(&bool_coord, COLOR, coord.x, coord.y);

                frame = Instant::now();
            }
        }
    }
}
