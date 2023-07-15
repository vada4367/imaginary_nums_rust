use crate::Complex;
use crate::Imaginary;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct Coord {
    pub matrix: Vec<Complex>,
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coord")
            .field("matrix", &self.matrix)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[allow(dead_code)]
impl Coord {
    pub fn init(x1: f64, y1: f64, x2: f64, y2: f64, term_x: i32, term_y: i32) -> Coord {
        let mut result: Vec<Complex> = vec![];

        for i in 0..term_x * term_y {
            result.push(Complex {
                a: x1 + (x2 - x1) / ((term_x) as f64) * ((i % term_x) as f64),
                b: Imaginary {
                    i: y1 + (y2 - y1) / ((term_y) as f64) * ((i / term_x) as f64),
                },
            });
        }

        Coord {
            matrix: result,
            x: term_x,
            y: term_y,
        }
    }

    fn dot_mand(&self, i: usize) -> bool {
        let c: Complex = self.matrix[i];
        let mut z: Complex = c;
        let iters: i32 = 100;
        let mut count: i32 = 0;

        while count <= iters {
            z = z * z + c;
            if z.a * z.a + z.b.i * z.b.i > 4. {
                break;
            }
            count += 1;
        }

        count > iters - 1
    }

    async fn part_mand(&self, point1: i32, point2: i32) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];

        for i in point1..point2 {
            result.push(self.dot_mand(i as usize));
        }

        result
    }

    pub async fn handle_mand(&self, handles: i32, i: i32) -> Vec<bool> {
        let (point1, point2): (i32, i32) = (
            (self.x * self.y as i32 / handles * i),
            self.x * self.y as i32 / handles * (i + 1),
        );

        let part: Vec<bool> = self.part_mand(point1, point2).await;

        part
    }

    pub async fn mand(self, handles: i32) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];
        for _ in 0..self.x * self.y {
            result.push(false);
        }

        let mut program_handles = vec![];

        for i in 0..handles {
            let arc_self = Arc::new(self.clone());

            let handle = tokio::spawn(async move {
                let copy_self = Arc::try_unwrap(arc_self).unwrap();
                let part = copy_self.handle_mand(handles, i).await;

                let (point1, point2): (i32, i32) = (
                    (copy_self.x * copy_self.y as i32 / handles * i),
                    copy_self.x * copy_self.y as i32 / handles * (i + 1),
                );

                (part, point1, point2)
            });

            program_handles.push(handle);
        }

        for handle in program_handles {
            let (part, point1, point2) = handle.await.unwrap();

            for i in point1..point2 {
                result[i as usize] = part[(i - point1) as usize];
            }
        }

        result
    }
}

pub fn print_mand(bool_coord: &Vec<bool>, c: char, x: i32, y: i32) {
    print!("\x1B[2J\x1B[1;1H"); // clear terminal

    for i in 0..x * y {
        match bool_coord[(i) as usize] {
            true => print!("{}", c),
            false => print!(" "),
        }

        if i % x == 0 {
            println!();
        }
    }
}
