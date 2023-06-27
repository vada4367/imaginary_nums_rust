use crate::Complex;
use crate::Imaginary;

pub struct Coord {
    pub matrix: Vec<Vec<Complex>>,
}

#[allow(dead_code)]
impl Coord {
    pub fn init(x1: f64, y1: f64, x2: f64, y2: f64, term_x: i32, term_y: i32) -> Coord {
        let mut result: Vec<Vec<Complex>> = vec![];

        for i in 0..term_x {
            let mut str: Vec<Complex> = vec![];
            for j in 0..term_y {
                str.push(Complex {
                    a: x1 + (x2 - x1) / ((term_x) as f64) * (i as f64),
                    b: Imaginary {
                        i: y1 + (y2 - y1) / ((term_y) as f64) * (j as f64),
                    },
                });
            }
            result.push(str);
        }

        Coord { matrix: result }
    }

    fn dot_mond(c: &Complex) -> bool {
        let mut z: Complex = *c;
        let iters: i32 = 100;
        let mut count: i32 = 0;

        while count <= iters {
            z = z * z + *c;
            if Complex::to_polar(&z).s > 2. {
                break;
            }
            count += 1;
        }

        count >= iters - 2
        //Complex::to_polar(&z).s < Complex::to_polar(c).s
    }

    pub fn mond(&self) -> Vec<Vec<bool>> {
        let mut result: Vec<Vec<bool>> = vec![];

        for i in 0..self.matrix.len() {
            let mut str: Vec<bool> = vec![];
            for j in 0..self.matrix[0].len() - 1 {
                str.push(Self::dot_mond(&self.matrix[i][j]));
            }
            result.push(str);
        }

        result
    }
}
