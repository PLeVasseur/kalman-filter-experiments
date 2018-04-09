extern crate rudie;
extern crate typenum;

use rudie::KalmanFilterMatGen2;
use typenum::consts::*;

fn main() {
    let kf_mg: KalmanFilterMatGen2<f32, U6> = KalmanFilterMatGen2::init();

    println!("{:?}", kf_mg);


    println!("Hello, world!");


}
