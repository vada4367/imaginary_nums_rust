mod complex;
//use crate::complex::pif;
use crate::complex::mond;
use crate::mond::Coord;
use crate::complex::Complex;
use crate::complex::imaginary::Imaginary;
use std::time::{Duration, Instant};

// Pith const's
#[allow(dead_code)]
const N :i32 = 10;

// Mond const's
const X :f64 = -0.55050055;
const Y :f64 = -0.5505;
const A :i32 = 100; // Size a from shape of print


fn print_mond(bool_coord :&Vec<Vec<bool>>) {
    for j in 0..bool_coord[0].len()-1 {
        for i in 0..bool_coord.len() {
            if bool_coord[i][j] { 
                print!("##"); 
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    //println!("{:#?}", pif::pith_trips(N));

    let ten_millis = Duration::from_millis(10); // Frame
    
    let mut size :f64 = 2.; // Scale of coord network
    let mut frame = Instant::now();
    loop { // loop scale
        if Instant::now() - frame > ten_millis {
            size /= 1.01;
            let coord :Coord = mond::Coord::init( // coord network
                X-size/2., 
                Y-size/2., 
                X+size/2., 
                Y+size/2.,
                A);
            let bool_coord :Vec<Vec<bool>> = coord.mond(); // Which dots is mond
            print_mond(&bool_coord);
            
            frame = Instant::now();
        }
    }
}
