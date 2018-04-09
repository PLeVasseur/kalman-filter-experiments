extern crate rudie;
extern crate typenum;

use rudie::{KalmanFilter};
use typenum::consts::*;

fn main() {
    let kf: KalmanFilter<f32, U6, U3, U0> = KalmanFilter::init();

    println!("{:?}", kf);
}
