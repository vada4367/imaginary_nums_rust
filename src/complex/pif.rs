use crate::Complex;
use crate::complex::Polar;
use crate::Imaginary;

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
                        powered.a.round(), 
                        powered.b.i.round(), 
                        complex_polar.s*complex_polar.s.round()]);
        }
    }
    result
}
