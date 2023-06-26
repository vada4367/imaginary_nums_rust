mod complex;
use crate::complex::pif;
use crate::complex::mond;
use crate::mond::Coord;
use crate::complex::Complex;
use crate::complex::imaginary::Imaginary;
use std::{thread, time};

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
    //println!("{:#?}", pif::pith_trips(10));

    let ten_millis = time::Duration::from_millis(10);
    let mut scale :f64;
    let x :f64 = -0.55;
    let y :f64 = -0.55;
    let mut size :f64 = 2.;
    loop {
        size /= 1.01;
        let coord :Coord = mond::Coord::init(
            x-size/2., 
            y-size/2., 
            x+size/2., 
            y+size/2.);
        let bool_coord :Vec<Vec<bool>> = coord.mond();
        print_mond(&bool_coord);
        thread::sleep(ten_millis);
    }
}
