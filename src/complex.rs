pub mod imaginary; // incluse imaginaries nums (b in (a + bi))
pub mod pif;
pub mod mond;
use core::ops::Mul; // include traits of operatior overloading
use core::ops::Div;
use core::ops::Sub;
use std::ops::Add;

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
        let _c :f64 = (num.b.i/num.a).atan()*180./PI; // corner 
        Polar {s : _s, c : _c} // return Polar record
    }

    pub fn print(num :&Complex) { // print a and bi function
        print!("a: {}, b: {}", 
                 num.a.round(),
                 num.b.i.round());
    }
    
    pub fn pow(&self, n :i32) -> Complex {
        let mut result :Complex = *self;
        for _ in 0..n { result = result*result; }
        result
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

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, num :Complex) -> Complex {
        Complex {
            a : Polar::to_complex(&(Complex::to_polar(&self)*Complex::to_polar(&num))).a,
            b : imaginary::Imaginary { 
                i : Polar::to_complex(&(Complex::to_polar(&self)*Complex::to_polar(&num))).b.i 
            },
        }
    }
}

#[allow(dead_code)]
impl Polar { // Polar functions
    pub fn to_complex(num :&Polar) -> Complex { // convert Polar to Complex struct
        let _a :f64 = num.s*((num.c)/180.*PI).cos(); // s is size 
        let _b :f64 = num.s*((num.c)/180.*PI).sin();       // c is corner
        Complex {a : _a, b : imaginary::Imaginary {i : _b}} // return struct
    }

    pub fn print(num :&Polar) {
        println!("s: {}, c: {}", num.s.round(), num.c.round());    
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
