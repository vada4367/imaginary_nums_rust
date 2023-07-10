use crate::Complex;
use crate::Imaginary;

#[allow(dead_code)]
pub fn pyth_trips(a: i32) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = vec![];
    for i in 1..a {
        for j in 1..i {
            let complex: Complex = Complex {
                a: i as f64,
                b: Imaginary { i: j as f64 },
            };

            let powered: Complex = complex * complex;

            result.push(vec![
                powered.a.round(),   // a
                powered.b.i.round(), // b
                (powered.a * powered.a + powered.b.i * powered.b.i)
                    .sqrt()
                    .round(), // c
            ]);
        }
    }
    result
}
