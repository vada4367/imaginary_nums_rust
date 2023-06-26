use crate::Complex;
use crate::complex::Polar;
use crate::Imaginary;
use round::round_down;

#[allow(dead_code)]
pub fn pith_trips(a :i32) -> Vec<Vec<f64>> {
    let mut result :Vec<Vec<f64>> = vec![];
    for i in 1..a {
        for j in 1..i {
            let complex :Complex = Complex {a : j as f64, b : Imaginary {i : i as f64}};
            let complex_polar :Polar = Complex::to_polar(&complex);
            let powered :Complex = 
                Polar::to_complex(&(complex_polar * complex_polar));
            result.push(vec![
                        round_down(powered.a, 0), 
                        round_down(powered.b.i, 0), 
                        round_down(complex_polar.s*complex_polar.s, 0)]);
        }
    }
    result
}
