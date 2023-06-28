const U: i32 = 1; // 0 - pyth trips, 1 - mand.

mod complex;
use crate::complex::imaginary::Imaginary;
use crate::complex::Complex;

// Pyth const's
const N: i32 = 10;

// Mand const's
const X: f64 = -0.552;
const Y: f64 = -0.5393;
const C: char = '␁'; // Color

fn print_mand(bool_coord: &Vec<Vec<bool>>) {
    print!("\x1B[2J\x1B[1;1H");

    for j in 0..bool_coord[0].len() - 1 {
        for i in 0..bool_coord.len() {
            if bool_coord[i][j] {
                print!("{}", C);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    if U == 0 {
        use crate::complex::pyth;
        println!("{:#?}", pyth::pyth_trips(N));
    }

    if U == 1 {
        use crate::complex::mand;
        use std::time::{Duration, Instant};

        let ten_millis = Duration::from_millis(30); // Frame time

        let mut scale: f64 = 0.05; // Scale of coord network
        let mut frame = Instant::now();

        loop {
            // loop scale
            if Instant::now() - frame > ten_millis {
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
                    let bool_coord: Vec<Vec<bool>> = coord.mond(); // Which dots is mond
                    print_mand(&bool_coord);

                    frame = Instant::now();
                } else {
                    println!("48 string ERROR");
                    break;
                }
            }
        }
    }
}
