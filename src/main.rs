mod complex;
use crate::complex::imaginary::Imaginary;
use crate::complex::Complex;
use clap::Parser;
use std::time::Duration;

#[derive(Debug)]
enum Mode {
    Mand,
    Pyth,
    Error,
}

impl Mode {
    fn from_string(str: &str) -> Result<Self, Self> {
        match str {
            "Mand" => return Ok(Mode::Mand),
            "Pyth" => return Ok(Mode::Pyth),
            _ => return Err(Mode::Error),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Mand"))]
    mode: String,

    #[arg(short, long, default_value_t = 10)]
    iters_pyth: i32,

    #[arg(short, default_value_t = -0.5505)]
    x: f64,
    #[arg(short, default_value_t = -0.53875)]
    y: f64,

    #[arg(short, long, default_value_t = 4)]
    handles: i32,
    #[arg(short, long, default_value_t = '#')]
    color: char,
    #[arg(short, long, default_value_t = 1.05)]
    speed: f64,
    #[arg(short, long, default_value_t = 30)]
    frame_time_millis: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match Mode::from_string(&args.mode).unwrap() {
        Mode::Pyth => {
            use crate::complex::pyth;
            println!("{:#?}", pyth::pyth_trips(args.iters_pyth));
        }
        Mode::Mand => {
            use crate::complex::mand;
            use std::time::Instant;

            let mut scale: f64 = 0.03; // Scale of coord network
            let mut frame = Instant::now();

            // loop scale
            loop {
                if Instant::now() - frame > Duration::from_millis(args.frame_time_millis) {
                    scale /= args.speed;

                    let (tx, ty): (usize, usize) = term_size::dimensions().unwrap();

                    let coord: mand::Coord = mand::Coord::init(
                        args.x - tx as f64 / 2. * scale * 2.,
                        args.y - ty as f64 / 2. * scale * 5.,
                        args.x + tx as f64 / 2. * scale * 2.,
                        args.y + ty as f64 / 2. * scale * 5.,
                        tx as i32,
                        ty as i32,
                    );

                    let bool_coord: Vec<bool> = coord.clone().mand(args.handles).await; // Which dots is mond

                    mand::print_mand(&bool_coord, args.color, coord.x, coord.y);

                    frame = Instant::now();
                }
            }
        }
        Mode::Error => todo!(),
    }
}
