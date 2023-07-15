pub mod imaginary; // include imaginaries nums (b in (a + bi))
pub mod mand;
pub mod pyth;

use std::fmt;
use core::ops::{Add, Div, Mul, Sub};


// Complex number (a + bi)

#[derive(Copy, Clone, Debug)]
pub struct Complex 
{
    pub a: f64,                  // a part
    pub b: imaginary::Imaginary, // bi part
}

// Polar complex record

#[derive(Copy, Clone)]
pub struct Polar 
{
    pub s: f64, // size (hypotenuse)
    pub c: f64, // corner
}


// functions (metods) for Complex struct
impl fmt::Display for Complex
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a: {}, b: {})", self.a, self.b.i)
    }
}

#[allow(dead_code)]
impl Complex 
{
    pub fn to_polar(num: &Complex) -> Polar {
        /* 
         * convert Complex Coordinate to Polar record
         * I treated a and bi as x1 and y1
         */
        let _s: f64 = (num.a * num.a + num.b.i * num.b.i).sqrt(); // (hypotenuse size)
        let _c: f64 = (num.b.i / num.a).atan().to_degrees(); // corner
        Polar { s: _s, c: _c } // return Polar record
    }

    pub fn pow(&self, n: i32) -> Complex {
        let mut result: Complex = *self;
        for _ in 0..n {
            result = result * result;
        }
        result
    }
}

// ops impls
impl Add<Complex> for Complex 
{
    type Output = Complex; // return struct Complex

    fn add(self: Complex, num: Complex) -> Complex {
        // num is second temp
        Complex {
            a: self.a + num.a, // a + a
            b: imaginary::Imaginary {
                i: self.b.i + num.b.i,
            }, // bi + bi
        }
    }
}

impl Sub<Complex> for Complex 
{
    type Output = Complex;

    fn sub(self: Complex, num: Complex) -> Complex {
        Complex {
            a: self.a - num.a,
            b: imaginary::Imaginary {
                i: self.b.i - num.b.i,
            },
        }
    }
}

impl Mul<Complex> for Complex 
{
    type Output = Complex;

    fn mul(self, num: Complex) -> Complex {
        Complex {
            a: self.a * num.a - self.b.i * num.b.i,
            b: imaginary::Imaginary {
                i: self.b.i * num.a + self.a * num.b.i,
            },
        }
    }
}


// Polar functions

impl fmt::Display for Polar
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s: {}, c: {})", self.s, self.c)
    }
}

#[allow(dead_code)]
impl Polar 
{
    // convert Polar to Complex record
    pub fn to_complex(num: &Polar) -> Complex {
        let _a: f64 = num.s * (num.c.to_radians()).cos(); // s is size
        let _b: f64 = num.s * (num.c.to_radians()).sin(); // c is corner
        Complex {
            a: _a,
            b: imaginary::Imaginary { i: _b },
        } // return struct
    }
}

impl Div<Polar> for Polar {
    type Output = Polar;

    fn div(self, num: Polar) -> Polar {
        Polar {
            s: self.s / num.s, /* f*cking math magic */
            c: self.c - num.c,
        }
    }
}
