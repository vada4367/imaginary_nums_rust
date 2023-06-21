pub mod imaginary; // incluse imaginaries nums (b in (a + bi))
pub mod pif;
use core::ops::Mul; // include traits of operatior overloading
use core::ops::Div;
use core::ops::Sub;
use std::ops::Add;
use round::round_down;

const PI :f64 = std::f64::consts::PI; // Pi const

#[derive(Copy, Clone)]
pub struct Complex { // Complex number (a + bi)
    pub a :f64, // a part
    pub b :imaginary::Imaginary, // bi part
}

#[derive(Copy, Clone)]
pub struct Polar { // Polar complex record
    pub s :f64, // size (hypotenuse)
    pub c :f64, // corner
}

#[allow(dead_code)]
impl Complex { // functions (metods) for Complex struct
    pub fn to_polar(num :&Complex) -> Polar { // convert Complex to Polar record
            /* i treated a and bi as x1 y1 */
        let _s :f64 = (num.a*num.a + num.b.i*num.b.i).sqrt(); // (hypotenuse size)
        let _c :f64 = (num.a/_s).asin()*180./PI; // corner 
        Polar {s : _s, c : _c} // return Polar record
    }

    pub fn print(num :&Complex) { // print a and bi function
        print!("a: {}, b: {}", 
                 round_down(num.a, 0),
                 round_down(num.b.i, 0));
    }
}

impl Add<Complex> for Complex { // ops
    type Output = Complex; // return struct Complex

    fn add(self :Complex, num :Complex) -> Complex { // num is second temp
        Complex {
            a : self.a + num.a, // a + a
            b : imaginary::Imaginary {i : self.b.i + num.b.i}, // bi + bi
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self :Complex, num :Complex) -> Complex {
        Complex {
            a : self.a - num.a,
            b : imaginary::Imaginary {i : self.b.i - num.b.i},
        }
    }
}

#[allow(dead_code)]
impl Polar { // Polar functions
    pub fn to_complex(num :&Polar) -> Complex { // convert Polar to Complex struct
        let _a :f64 = num.s*(num.c/180.*PI).sin(); // s is size 
        let _b :f64 = num.s*((num.c)/180.*PI).cos();       // c is corner
        Complex {a : _a, b : imaginary::Imaginary {i : _b}} // return struct
    }

    pub fn print(num :&Polar) {
        println!("s: {}, c: {}", num.s, num.c);    
    }
}

impl Mul<Polar> for Polar {
    type Output = Polar;

    fn mul(self, num: Polar) -> Polar {
        Polar {
            s : self.s * num.s, /* f*cking magic */
            c : self.c + num.c,
        }
    }
}

impl Div<Polar> for Polar {
    type Output = Polar;

    fn div(self, num: Polar) -> Polar {
        Polar {
            s : self.s / num.s, /* f*cking magik */
            c : self.c - num.c,
        }
    }
}
