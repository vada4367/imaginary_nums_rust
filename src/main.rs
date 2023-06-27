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
const X :f64 = -0.552;
const Y :f64 = -0.5393;
const C :char = '␁'; // Color

fn print_mond(bool_coord :&Vec<Vec<bool>>) {
    print!("\x1B[2J\x1B[1;1H");

    for j in 0..bool_coord[0].len()-1 {
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
    //println!("{:#?}", pif::pith_trips(N));

    let ten_millis = Duration::from_millis(30); // Frame
    
    let mut scale :f64 = 0.01; // Scale of coord network
    let mut frame = Instant::now();

    loop { // loop scale
        if Instant::now() - frame > ten_millis {
            scale /= 1.01;
            
            if let Some((tx, ty)) = term_size::dimensions() {
                let coord :Coord = mond::Coord::init( // coord network
                    X-tx as f64/2.*scale*2., 
                    Y-ty as f64/2.*scale*5., 
                    X+tx as f64/2.*scale*2., 
                    Y+ty as f64/2.*scale*5.,
                    tx as i32,
                    ty as i32);
                let bool_coord :Vec<Vec<bool>> = coord.mond(); // Which dots is mond
                print_mond(&bool_coord);
                
                frame = Instant::now();


            } else {println!("48 string ERROR");break;}
        }
    }
}
